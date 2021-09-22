use std::time::Duration;

use super::pwm::PWMControl;
use super::timer::USTimer;

pub struct HallSensors {
    pub a: HallSensor,
    pub b: HallSensor,
    pub c: HallSensor,
}

impl HallSensors {
    pub fn new() -> Self {
        Self {
            a: HallSensor::new(),
            b: HallSensor::new(),
            c: HallSensor::new(),
        }
    }

    pub fn update(&mut self, _now: Duration) {
        todo!();
    }
}

pub struct HallSensor;

impl HallSensor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn at_rise<F: Fn(&mut PWMControl, f32, Option<&mut USTimer>) -> ()>(
        &self,
        pwms: &mut PWMControl,
        vr_adc: f32,
        ut: Option<&mut USTimer>,
        f: F,
    ) {
        match ut {
            Some(ut) => f(pwms, vr_adc, Some(ut)),
            None => f(pwms, vr_adc, None),
        }
    }

    pub fn at_fall<F: Fn(&mut PWMControl) -> ()>(&self, pwms: &mut PWMControl, f: F) {
        f(pwms);
    }
}
