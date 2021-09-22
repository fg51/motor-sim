use std::time::Duration;

pub struct WorldClock {
    duration: Duration,
    now: Duration,
}

impl WorldClock {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            now: Duration::from_micros(0),
        }
    }

    pub fn update(&mut self) {
        self.now += self.duration;
    }
}
