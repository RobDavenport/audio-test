use std::f32::consts::PI;

#[derive(Clone)]
pub enum FeedbackLevel {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
}

impl Default for FeedbackLevel {
    fn default() -> Self {
        Self::Zero
    }
}

impl FeedbackLevel {
    pub fn as_multiplier(&self) -> f32 {
        match self {
            FeedbackLevel::Zero => 0.0,
            FeedbackLevel::One => PI / 128.0,
            FeedbackLevel::Two => PI / 64.0,
            FeedbackLevel::Three => PI / 32.0,
            FeedbackLevel::Four => PI / 16.0,
            FeedbackLevel::Five => PI / 8.0,
            FeedbackLevel::Six => PI / 4.0,
            FeedbackLevel::Seven => PI / 2.0,
            FeedbackLevel::Eight => PI,
            FeedbackLevel::Nine => PI * 2.0,
            FeedbackLevel::Ten => PI * 4.0,
            FeedbackLevel::Eleven => PI * 8.0,
            FeedbackLevel::Twelve => PI * 16.0,
            FeedbackLevel::Thirteen => PI * 32.0,
            FeedbackLevel::Fourteen => PI * 64.0,
            FeedbackLevel::Fifteen => PI * 128.0,
        }
    }
}
