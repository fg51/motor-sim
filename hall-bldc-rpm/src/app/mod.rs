use crate::device::hallsensor::HallSensors;
use crate::device::pwm::PWMControl;
use crate::device::timer::{wait_ms, USTimer};

const START: u32 = 8;

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

pub fn run(
    fc: &mut ForcedCommutation,
    hs: &mut HallSensors,
    pwms: &mut PWMControl,
    ut: &mut USTimer,
    vadc: &Vadc,
) {
    let target_speed = TargetSpeed::new(vadc.read_v_adc()); //  Vr_adc=V_adc.read();

    ut.start();

    if target_speed.is_high() {
        if fc.is_no_counted() {
            while fc.is_low_speed() {
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

                fc.countup();
            }
        }
    }

    hs.a.at_rise(pwms, target_speed.vr_adc, None, |pwms, vr_adc, _ut1| {
        pwms.hc_high(vr_adc)
    }); //HAH HA.rise(&HCH);  //HAH
    hs.c.at_fall(pwms, |pwms| pwms.hb_low()); //  HC.fall(&HBL);  //HCL
    hs.b.at_rise(pwms, target_speed.vr_adc, Some(ut), |pwms, vr_adc, ut1| {
        pwms.ha_high(vr_adc, &mut ut1.unwrap())
    }); //HBH HB.rise(&HAH);  //HBH
    hs.a.at_fall(pwms, |pwms| pwms.hc_low()); //  HA.fall(&HCL);  //HAL
    hs.c.at_rise(pwms, target_speed.vr_adc, None, |pwms, vr_adc, _ut1| {
        pwms.hb_high(vr_adc)
    }); //  HC.rise(&HBH);  //HCH
    hs.b.at_fall(pwms, |pwms| pwms.ha_low()); //  HB.fall(&HAL);  //HBL

    //  //  s=0;
    if target_speed.is_low() {
        fc.reset();
    }

    let us_i = (pwms.ut2() - pwms.ut1()).abs();
    let num_of_pair = 7;
    let rps = 1. / (num_of_pair as f32 * us_i * 1E-6);
    let speed_rpm = 60. * rps;
    println!("{:.3} , {:.3} ", speed_rpm, target_speed.vr_adc); // pc.printf("%.3f , %.3f \r" ,Speed ,Vr_adc);

    //   //UP=HA; VP=HB; WP=HC;
    //   // pc.printf("%d  ,%d ,%d\r" ,UP,VP,WP);
    //   myled = !myled;
}

pub struct Vadc;

impl Vadc {
    pub fn read_v_adc(&self) -> f32 {
        todo!();
    }
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
