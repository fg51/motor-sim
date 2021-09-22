use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct Freq(u64);

impl Freq {
    pub const fn new(v: u64) -> Self {
        Self(v)
    }
    pub fn into_duration(&self) -> Duration {
        Duration::from_secs(1 / self.0)
    }
}

pub trait ExtFreq {
    fn hz(self) -> Freq;
    fn khz(self) -> Freq;
}

impl ExtFreq for u64 {
    fn hz(self) -> Freq {
        Freq::new(self)
    }

    fn khz(self) -> Freq {
        Freq::new(self * 1000)
    }
}

impl ExtFreq for u32 {
    fn hz(self) -> Freq {
        Freq::new(self as u64)
    }

    fn khz(self) -> Freq {
        Freq::new(self as u64 * 1000)
    }
}
