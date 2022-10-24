#![allow(dead_code)]

use hal::rcc::Rcc;
use hal::timer::pwm::*;
use hal::timer::*;

use crate::*;

pub struct MotorControl {
    enable: MotorDriverEnable,
    fault: MotorDriverFault,
    tork: PwmPin<TIM16, Channel1>,
    pwm_a: PwmPin<TIM1, Channel4>,
    pwm_b: PwmPin<TIM1, Channel1>,
    phase_a: MotorAPhase,
    phase_b: MotorBPhase,
}

impl MotorControl {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        speed_tim: TIM1,
        tork_tim: TIM16,
        tork: MotorDriverTork,
        enable: MotorDriverEnable,
        fault: MotorDriverFault,
        pwm_a: MotorAPwm,
        pwm_b: MotorBPwm,
        phase_a: MotorAPhase,
        phase_b: MotorBPhase,
        rcc: &mut Rcc,
    ) -> Self {
        let tork_pwm = tork_tim.pwm(10.kHz(), rcc);
        let mut tork = tork_pwm.bind_pin(tork);
        tork.set_duty(tork.get_max_duty());
        tork.enable();

        let speed_pwm = speed_tim.pwm(100.Hz(), rcc);
        let mut pwm_a = speed_pwm.bind_pin(pwm_a);
        pwm_a.set_duty(0);
        pwm_a.enable();

        let mut pwm_b = speed_pwm.bind_pin(pwm_b);
        pwm_b.set_duty(0);
        pwm_b.enable();

        Self {
            tork,
            enable,
            fault,
            pwm_a,
            pwm_b,
            phase_a,
            phase_b,
        }
    }
}
