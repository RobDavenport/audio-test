TODO:
1. Save "instruments" ? Combination of patches
    - Done via UI?
1. Sequencer?
1. Effects
1. "Noise" frequency using LFSR and pre-defined periods/freqs?
1. SSG-EG mode?
1. Optimizations:
    - Optimize sampler tick rates/tick times with integer math?
    - Optimize sine function?
    - Use fixed point? u16 or something instead of f32, can then use a lookup table 

https://plutiedev.com/ym2612-registers#reg-40
https://www.smspower.org/maxim/Documents/YM2612#alittlebitaboutoperators
https://manualmachine.com/sega/genesisfmdrive/2121418-user-guide/
https://moddingwiki.shikadi.net/wiki/OPL_chip
http://gendev.spritesmind.net/forum/viewtopic.php?t=386&start=106

ym2612 feedbacks:
0 = 0
1 = PI / 16
2 = PI / 8
3 = PI / 4
4 = PI / 2
5 = PI
6 = 2 * PI
7 = 4 * PI

consider fast sin function as described:
https://www.youtube.com/watch?v=1xlCVBIF_ig