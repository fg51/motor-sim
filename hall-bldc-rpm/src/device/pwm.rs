use std::time::Duration;

use crate::values::freq::Freq;

use super::timer::USTimer;

pub struct PWM {
    _freq: Freq,
    duration: Duration,
    is_high: bool,
    duty: Option<f32>,
    next_time: Duration,
}

impl PWM {
    pub fn new(freq: Freq) -> Self {
        Self {
            _freq: freq.clone(),
            duration: freq.into_duration(),
            is_high: false,
            duty: None,
            next_time: Duration::from_micros(0),
        }
    }

    pub fn write(&mut self, duty: f32) {
        if duty == 0.0 {
            self.duty = None;
            self.is_high = false;
        } else {
            self.duty = Some(duty);
        }
    }

    fn on_duration(&self, duty: f64) -> Duration {
        Duration::from_micros((self.duration.as_micros() as f64 * duty as f64) as u64)
    }

    fn off_duration(&self, duty: f64) -> Duration {
        Duration::from_micros((self.duration.as_micros() as f64 * (1. - duty as f64)) as u64)
    }

    pub fn update(&mut self, now: &Duration) {
        match self.duty {
            None => (),
            Some(duty) => {
                if *now < self.next_time {
                    return ();
                }
                if self.is_high {
                    self.is_high = false;
                    self.next_time = *now + self.off_duration(duty as f64);
                } else {
                    self.is_high = true;
                    self.next_time = *now + self.on_duration(duty as f64);
                }
            }
        }
    }
}

pub struct PWMControl {
    r: u32,
    ut1: f32,
    ut2: f32,
    pub pwm_a: PWM,
    pub pwm_b: PWM,
    pub pwm_c: PWM,
}

impl PWMControl {
    pub fn ut1(&self) -> f32 {
        self.ut1
    }

    pub fn ut2(&self) -> f32 {
        self.ut2
    }

    // HAH
    pub fn ha_high(&mut self, vr_adc: f32, us_timer: &mut USTimer) {
        let s = self.r % 2;
        if s == 0 {
            self.ut1 = us_timer.read_us(); //     ut1 = uT.read_us();
            self.r += 1;
        }

        if s == 1 {
            self.ut2 = us_timer.read_us();
            self.r += 1;
            us_timer.reset();
        }
        self.pwm_a.write(vr_adc);
        self.pwm_b.write(0.);
        self.pwm_c.write(0.);
    }

    // HAL
    pub fn ha_low(&mut self) {
        self.pwm_a.write(0.);
        self.pwm_c.write(0.);
    }

    // HBH
    pub fn hb_high(&mut self, vr_adc: f32) {
        self.pwm_a.write(0.);
        self.pwm_b.write(vr_adc);
        self.pwm_c.write(0.);
    }

    // HBL
    pub fn hb_low(&mut self) {
        self.pwm_a.write(0.);
        self.pwm_b.write(0.);
    }

    // HCH
    pub fn hc_high(&mut self, vr_adc: f32) {
        self.pwm_a.write(0.);
        self.pwm_b.write(0.);
        self.pwm_c.write(vr_adc);
    }

    // HCL
    pub fn hc_low(&mut self) {
        self.pwm_b.write(0.);
        self.pwm_c.write(0.);
    }
}
