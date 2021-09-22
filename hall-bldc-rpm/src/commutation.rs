#[derive(Default)]
pub struct ForcedCommutation {
    count: u32,
}

impl ForcedCommutation {
    const LOW_SPEED_MAX: u32 = 50;

    pub fn is_no_counted(&self) -> bool {
        self.count == 0
    }

    pub fn is_low_speed(&self) -> bool {
        self.count < Self::LOW_SPEED_MAX
    }

    pub fn countup(&mut self) {
        self.count += 1;
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }
}
