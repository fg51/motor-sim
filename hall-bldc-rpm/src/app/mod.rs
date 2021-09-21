const START: u32 = 8;

pub fn run(q: &mut u32, pwm: &mut PWMControl, ut: &mut USTimer) {
    let vr_adc = read_v_adc(); //  Vr_adc=V_adc.read();

    ut.start();

    if vr_adc > 0.15 {
        if *q == 0 {
            while *q < 50 {
                pwm.pwm_a.write(0.);
                pwm.pwm_b.write(0.5);
                pwm.pwm_c.write(0.);
                wait_ms(START);

                pwm.pwm_a.write(0.5);
                pwm.pwm_b.write(0.);
                pwm.pwm_c.write(0.);
                wait_ms(START);

                pwm.pwm_a.write(0.);
                pwm.pwm_b.write(0.);
                pwm.pwm_c.write(0.5);
                wait_ms(START);

                *q += 1;
            }
        }
    }

    //  HA.rise(&HCH);  //HAH
    //  HC.fall(&HBL);  //HCL
    //  HB.rise(&HAH);  //HBH
    //  HA.fall(&HCL);  //HAL
    //  HC.rise(&HBH);  //HCH
    //  HB.fall(&HAL);  //HBL
    //  //  s=0;
    if vr_adc < 0.1 {
        *q = 0;
    }

    let us_i = (pwm.ut2() - pwm.ut1()).abs();
    let rps = 1. / (7.0 * us_i * 1E-6);
    let speed_rpm = 60. * rps;
    println!("{:.3} , {:.3} ", speed_rpm, vr_adc); // pc.printf("%.3f , %.3f \r" ,Speed ,Vr_adc);

    //   //UP=HA; VP=HB; WP=HC;
    //   // pc.printf("%d  ,%d ,%d\r" ,UP,VP,WP);
    //   myled = !myled;
}

pub fn read_v_adc() -> f32 {
    todo!();
}

pub struct PWM {}

impl PWM {
    pub fn write(&mut self, _x: f32) {
        todo!();
    }
}

pub struct PWMControl {
    r: u32,
    ut1: f32,
    ut2: f32,
    pwm_a: PWM,
    pwm_b: PWM,
    pwm_c: PWM,
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
