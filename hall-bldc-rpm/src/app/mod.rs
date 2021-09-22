use crate::commutation::ForcedCommutation;
use crate::device::adc::Vadc;
use crate::device::hallsensor::HallSensors;
use crate::device::pwm::PWMControl;
use crate::device::timer::{wait_ms, USTimer};
use crate::device::world::WorldClock;
use crate::domain::TargetSpeed;

use std::time::Duration;

const START: Duration = Duration::from_millis(8); //[msec]

pub fn main_loop() {
    let world = WorldClock::new(Duration::from_millis(1));
    // initialize();
    loop {
        //run();
    }

    // update
    // world_clock.update();
    //
    // PWMControl.update(&world_clock);
    // pwm_a.update();
    // pwm_b.update();
    // pwm_c.update();
    //
    // HallSensors.update(&world_clock);
    // ut.update(&world_clock);
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

    hs.a.at_rise(pwms, target_speed.vr_adc(), None, |pwms, vr_adc, _ut1| {
        pwms.hc_high(vr_adc)
    }); //HAH HA.rise(&HCH);  //HAH
    hs.c.at_fall(pwms, |pwms| pwms.hb_low()); //  HC.fall(&HBL);  //HCL
    hs.b.at_rise(
        pwms,
        target_speed.vr_adc(),
        Some(ut),
        |pwms, vr_adc, ut1| pwms.ha_high(vr_adc, &mut ut1.unwrap()),
    ); //HBH HB.rise(&HAH);  //HBH
    hs.a.at_fall(pwms, |pwms| pwms.hc_low()); //  HA.fall(&HCL);  //HAL
    hs.c.at_rise(pwms, target_speed.vr_adc(), None, |pwms, vr_adc, _ut1| {
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
    println!("{:.3} , {:.3} ", speed_rpm, target_speed.vr_adc()); // pc.printf("%.3f , %.3f \r" ,Speed ,Vr_adc);

    //   //UP=HA; VP=HB; WP=HC;
    //   // pc.printf("%d  ,%d ,%d\r" ,UP,VP,WP);
    //   myled = !myled;
}
