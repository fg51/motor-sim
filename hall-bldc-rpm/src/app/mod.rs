const START: u32 = 8;

pub struct ForceRotate {
    count: u32,
}

impl ForceRotate {
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

pub fn run(q: &mut ForceRotate, hs: &mut HallSensors, pwms: &mut PWMControl, ut: &mut USTimer) {
    let vr_adc = read_v_adc(); //  Vr_adc=V_adc.read();
                               // let target_speed = vr_adc;

    ut.start();

    if vr_adc > 0.15 {
        if q.is_no_counted() {
            while q.is_low_speed() {
                let (duty, off) = (0.5, 0.);
                pwms.pwm_a.write(off);
                pwms.pwm_b.write(duty);
                pwms.pwm_c.write(off);
                wait_ms(START);

                pwms.pwm_a.write(duty);
                pwms.pwm_b.write(off);
                pwms.pwm_c.write(off);
                wait_ms(START);

                pwms.pwm_a.write(off);
                pwms.pwm_b.write(off);
                pwms.pwm_c.write(duty);
                wait_ms(START);

                q.countup();
            }
        }
    }

    hs.a.at_rise(pwms, None, |pwms, _ut1| pwms.hc_high(vr_adc)); //HAH HA.rise(&HCH);  //HAH
    hs.c.at_fall(pwms, |pwms| pwms.hb_low()); //  HC.fall(&HBL);  //HCL
    hs.b.at_rise(pwms, Some(ut), |pwms, ut1| {
        pwms.ha_high(vr_adc, &mut ut1.unwrap())
    }); //HBH HB.rise(&HAH);  //HBH
    hs.a.at_fall(pwms, |pwms| pwms.hc_low()); //  HA.fall(&HCL);  //HAL
    hs.c.at_rise(pwms, None, |pwms, _ut1| pwms.hb_high(vr_adc)); //  HC.rise(&HBH);  //HCH
    hs.b.at_fall(pwms, |pwms| pwms.ha_low()); //  HB.fall(&HAL);  //HBL

    //  //  s=0;
    if vr_adc < 0.1 {
        q.reset();
    }

    let us_i = (pwms.ut2() - pwms.ut1()).abs();
    let num_of_pair = 7;
    let rps = 1. / (num_of_pair as f32 * us_i * 1E-6);
    let speed_rpm = 60. * rps;
    println!("{:.3} , {:.3} ", speed_rpm, vr_adc); // pc.printf("%.3f , %.3f \r" ,Speed ,Vr_adc);

    //   //UP=HA; VP=HB; WP=HC;
    //   // pc.printf("%d  ,%d ,%d\r" ,UP,VP,WP);
    //   myled = !myled;
}

pub struct HallSensors {
    pub a: HallSensor,
    pub b: HallSensor,
    pub c: HallSensor,
}

pub struct HallSensor;

impl HallSensor {
    pub fn at_rise<F: Fn(&mut PWMControl, Option<&mut USTimer>) -> ()>(
        &self,
        pwms: &mut PWMControl,
        ut: Option<&mut USTimer>,
        f: F,
    ) {
        match ut {
            Some(ut) => f(pwms, Some(ut)),
            None => f(pwms, None),
        }
    }

    pub fn at_fall<F: Fn(&mut PWMControl) -> ()>(&self, pwms: &mut PWMControl, f: F) {
        f(pwms);
    }
}

pub fn read_v_adc() -> f32 {
    todo!();
}

//pub struct Duty(pub f32);
//
//impl Duty {
//    pub fn new(v: f32) -> Result<Self> {
//        if v > 1.0 {
//            todo!();
//        }
//        if v < 0. {
//            todo!();
//        }
//        Ok(Self(v))
//    }
//}

pub struct PWM {}

impl PWM {
    pub fn write(&mut self, _duty: f32) {
        todo!();
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

pub struct USTimer;

impl USTimer {
    pub fn start(&mut self) {
        todo!();
    }

    pub fn reset(&mut self) {
        todo!();
    }

    pub fn read_us(&self) -> f32 {
        todo!();
    }
}

pub fn wait_ms(_msec: u32) {
    todo!();
}
