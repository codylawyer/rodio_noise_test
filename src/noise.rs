use rand::*;
use rand_distr::StandardNormal;
use rodio::*;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Noise {}

impl Noise {
    #[inline]
    pub fn new() -> Noise {
        Noise {}
    }
}

impl Iterator for Noise {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let mut rng = rand::thread_rng();
        let value: f32 = rng.sample(StandardNormal);
        Some(value)
    }
}

impl Source for Noise {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
