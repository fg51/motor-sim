use std::time::Duration;

#[derive(Default)]
pub struct USTimer {
    now: Duration,
    start: Duration,
}

impl USTimer {
    pub fn start(&mut self) {
        self.start = self.now;
    }

    pub fn reset(&mut self) {
        self.now = Duration::from_micros(0);
        self.start = Duration::from_micros(0);
    }

    pub fn read_us(&self) -> f32 {
        (self.now - self.start).as_nanos() as f32 / 1000.
    }

    pub fn update(&mut self, now: Duration) {
        self.now = now;
    }
}

pub fn wait_ms(_msec: Duration) {
    todo!();
}
