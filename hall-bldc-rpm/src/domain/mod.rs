pub struct TargetSpeed {
    vr_adc: f32,
}

impl TargetSpeed {
    pub fn new(vr_adc: f32) -> Self {
        Self { vr_adc }
    }

    pub fn is_high(&self) -> bool {
        self.vr_adc > 0.15
    }

    pub fn is_low(&self) -> bool {
        self.vr_adc < 0.1
    }

    pub fn vr_adc(&self) -> f32 {
        self.vr_adc
    }
}
