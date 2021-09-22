#[derive(Default)]
pub struct Vadc {
    v_adc: f32, // voltage at volume.
}

impl Vadc {
    pub fn read_v_adc(&self) -> f32 {
        self.v_adc
    }

    pub fn update(&mut self) {
        todo!();
    }
}
