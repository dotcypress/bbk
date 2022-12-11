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
    speed: (i8, i8),
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
        tork.set_duty(4_200);
        tork.enable();

        let mut pwm_tim = speed_tim.pwm(100.Hz(), rcc);
        pwm_tim.listen();

        let mut pwm_a = pwm_tim.bind_pin(pwm_a);
        pwm_a.set_duty(pwm_a.get_max_duty() / 4);
        pwm_a.enable();

        let mut pwm_b = pwm_tim.bind_pin(pwm_b);
        pwm_b.set_duty(pwm_a.get_max_duty() / 4);
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
            speed: (0, 0),
        }
    }

    pub fn fault_detected(&self) -> bool {
        self.fault.is_low().unwrap()
    }

    pub fn on(&mut self) {
        self.enable.set_high().ok();
    }

    pub fn off(&mut self) {
        self.enable.set_low().ok();
    }

    pub fn set_tork(&mut self, tork: u8) {
        self.tork.set_duty(4_200 - tork as u16 * 2);
    }

    pub fn set_speed(&mut self, left: i8, right: i8) {
        self.speed = (left, right);
    }

    pub fn tick(&mut self) {
        let target_duty = self.pwm_a.get_max_duty() as u32 * self.speed.0.abs() as u32 / 0x80;
        let duty = self.pwm_a.get_duty() as u32;
        let delta_a = target_duty.abs_diff(duty).clamp(0, 512);
        let duty = if target_duty > duty {
            duty + delta_a
        } else {
            duty.saturating_sub(delta_a)
        };
        self.pwm_a.set_duty(duty as _);
        self.phase_a
            .set_state(self.speed.0.is_positive().into())
            .ok();

        let target_duty = self.pwm_b.get_max_duty() as u32 * self.speed.1.abs() as u32 / 0x80;
        let duty = self.pwm_b.get_duty() as u32;
        let delta_b = target_duty.abs_diff(duty).clamp(0, 512);
        let duty = if target_duty > duty {
            duty + delta_b
        } else {
            duty.saturating_sub(delta_b)
        };
        self.pwm_b.set_duty(duty as _);
        self.phase_b
            .set_state(self.speed.1.is_positive().into())
            .ok();

        self.pwm_tim.clear_irq();
    }
}
