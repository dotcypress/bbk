use hal::rcc::Rcc;
use hal::timer::pwm::*;
use hal::timer::*;

use crate::*;

pub struct MotorControl {
    enable: MotorDriverEnable,
    fault: MotorDriverFault,
    phase_a: MotorAPhase,
    phase_b: MotorBPhase,
    pwm_tim: Pwm<TIM1>,
    pwm_a: PwmPin<TIM1, Channel4>,
    pwm_b: PwmPin<TIM1, Channel1>,
    tork: PwmPin<TIM16, Channel1>,
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
        let min_tork = 4_200;
        let max_tork = 3_800;
        tork.set_duty(3_900);
        tork.enable();

        let mut pwm_tim = speed_tim.pwm(100.Hz(), rcc);
        pwm_tim.listen();

        let mut pwm_a = pwm_tim.bind_pin(pwm_a);
        pwm_a.set_duty(pwm_a.get_max_duty()/8);
        pwm_a.enable();

        let mut pwm_b = pwm_tim.bind_pin(pwm_b);
        pwm_b.set_duty(0);
        pwm_b.enable();

        Self {
            tork,
            enable,
            fault,
            pwm_tim,
            pwm_a,
            pwm_b,
            phase_a,
            phase_b,
        }
    }

    pub fn tick(&mut self) {
        self.pwm_tim.clear_irq();
        if self.fault.is_low().unwrap() {}
    }

    pub fn on(&mut self) {
        self.enable.set_high().ok();
    }

    pub fn off(&mut self) {
        self.enable.set_low().ok();
    }
}
