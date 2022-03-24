use std::sync::Arc;

use egui::{ClippedMesh, Context, TexturesDelta, Ui};
use egui_wgpu_backend::{BackendError, RenderPass, ScreenDescriptor};
use parking_lot::RwLock;
use pixels::{wgpu, PixelsContext};
use winit::window::Window;

use crate::{
    patches::{PatchDefinition, OPERATOR_COUNT},
    Waveform,
};

/// Manages all state required for rendering egui over `Pixels`.
pub(crate) struct Framework {
    // State for egui.
    egui_ctx: Context,
    egui_state: egui_winit::State,
    screen_descriptor: ScreenDescriptor,
    rpass: RenderPass,
    paint_jobs: Vec<ClippedMesh>,
    textures: TexturesDelta,

    // State for the GUI
    gui: Gui,
}

/// Example application state. A real application will need a lot more state than this.
struct Gui {
    /// Only show the egui window when true.
    patch_handle: Arc<RwLock<PatchDefinition>>,
}

impl Framework {
    /// Create egui.
    pub(crate) fn new(
        width: u32,
        height: u32,
        scale_factor: f32,
        pixels: &pixels::Pixels,
        patch_handle: Arc<RwLock<PatchDefinition>>,
    ) -> Self {
        let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

        let egui_ctx = Context::default();
        let egui_state = egui_winit::State::from_pixels_per_point(max_texture_size, scale_factor);
        let screen_descriptor = ScreenDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor,
        };
        let rpass = RenderPass::new(pixels.device(), pixels.render_texture_format(), 1);
        let textures = TexturesDelta::default();
        let gui = Gui::new(patch_handle);

        Self {
            egui_ctx,
            egui_state,
            screen_descriptor,
            rpass,
            paint_jobs: Vec::new(),
            textures,
            gui,
        }
    }

    /// Handle input events from the window manager.
    pub(crate) fn handle_event(&mut self, event: &winit::event::WindowEvent) {
        self.egui_state.on_event(&self.egui_ctx, event);
    }

    /// Resize egui.
    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.physical_width = width;
            self.screen_descriptor.physical_height = height;
        }
    }

    /// Update scaling factor.
    pub(crate) fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.scale_factor = scale_factor as f32;
    }

    /// Prepare egui.
    pub(crate) fn prepare(&mut self, window: &Window) {
        // Run the egui frame and create all paint jobs to prepare for rendering.
        let raw_input = self.egui_state.take_egui_input(window);
        let output = self.egui_ctx.run(raw_input, |egui_ctx| {
            // Draw the demo application.
            self.gui.ui(egui_ctx);
        });

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(window, &self.egui_ctx, output.platform_output);
        self.paint_jobs = self.egui_ctx.tessellate(output.shapes);
    }

    /// Render egui.
    pub(crate) fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
    ) -> Result<(), BackendError> {
        // Upload all resources to the GPU.
        self.rpass
            .add_textures(&context.device, &context.queue, &self.textures)?;
        self.rpass.update_buffers(
            &context.device,
            &context.queue,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Record all render passes.
        self.rpass.execute(
            encoder,
            render_target,
            &self.paint_jobs,
            &self.screen_descriptor,
            None,
        )?;

        // Cleanup
        let textures = std::mem::take(&mut self.textures);
        self.rpass.remove_textures(textures)
    }
}

impl Gui {
    /// Create a `Gui`.
    fn new(patch_handle: Arc<RwLock<PatchDefinition>>) -> Self {
        Self { patch_handle }
    }

    /// Create the UI using egui.
    fn ui(&mut self, ctx: &Context) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.label("Patch Settings");

            let mut patch = self.patch_handle.write();
            ui.add(egui::Slider::new(&mut patch.feedback.0, 0..=15).text("Feedback"));
            ui.add(egui::Slider::new(&mut patch.algorithm.0, 0..=7).text("Algorithm"));
            drop(patch);

            (0..OPERATOR_COUNT).for_each(|index| self.operator(ui, index));
        });
    }

    fn operator(&mut self, ui: &mut Ui, index: usize) {
        ui.separator();

        let ref mut patch = self.patch_handle.write();
        let ref mut operator = patch.operators[index].write();

        ui.horizontal(|ui| {
            ui.selectable_value(&mut operator.waveform, Waveform::Sine, "Sine");
            ui.selectable_value(&mut operator.waveform, Waveform::HalfSine, "HalfSine");
            ui.selectable_value(
                &mut operator.waveform,
                Waveform::AbsoluteSine,
                "AbsoluteSine",
            );
            ui.selectable_value(&mut operator.waveform, Waveform::QuarterSine, "QuarterSine");
            ui.selectable_value(
                &mut operator.waveform,
                Waveform::AlternatingSine,
                "AlternatingSine",
            );
            ui.selectable_value(&mut operator.waveform, Waveform::CamelSine, "CamelSine");
            ui.selectable_value(&mut operator.waveform, Waveform::Square, "Square");
            ui.selectable_value(
                &mut operator.waveform,
                Waveform::LogarithmicSaw,
                "LogarithmicSaw",
            );
        });

        ui.add(
            egui::Slider::new(&mut operator.frequency_multiplier.0, 0..=16)
                .text("Frequency Multiuplier"),
        );

        // Envelope
        let ref mut envelope = operator.envelope.write();
        ui.horizontal(|ui| {
            ui.add(
                egui::Slider::new(&mut envelope.total_level, u8::MIN..=u8::MAX)
                    .text("TL")
                    .vertical(),
            );
            ui.add(
                egui::Slider::new(&mut envelope.attack_rate, u8::MIN..=u8::MAX)
                    .text("AR")
                    .vertical(),
            );
            ui.add(
                egui::Slider::new(&mut envelope.decay_attack_rate, u8::MIN..=u8::MAX)
                    .text("D1")
                    .vertical(),
            );
            ui.add(
                egui::Slider::new(&mut envelope.sustain_level, u8::MIN..=u8::MAX)
                    .text("SL")
                    .vertical(),
            );
            ui.add(
                egui::Slider::new(&mut envelope.decay_sustain_rate, u8::MIN..=u8::MAX)
                    .text("D2")
                    .vertical(),
            );
            ui.add(
                egui::Slider::new(&mut envelope.release_rate, u8::MIN..=u8::MAX)
                    .text("RR")
                    .vertical(),
            );
        });
    }
}
