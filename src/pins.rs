use crate::hal::gpio::{gpioa::*, gpiob::*, gpioc::*};
use hal::gpio::*;
use hal::prelude::*;
use hal::rcc::Rcc;
use hal::stm32::*;
use crate::*;

// Qwiic I2C
pub type I2cClk = PB8<Output<OpenDrain>>;
pub type I2cSda = PB9<Output<OpenDrain>>;

// Infared
pub type IrTx = PA4<DefaultMode>;
pub type IrRx = PC15<Input<Floating>>;

// Neopixel
pub type Neopixel = PA2<DefaultMode>;

// Power management
pub type Standby = PA6<Output<PushPull>>;
pub type BatterySense = PA3<DefaultMode>;

// Motor control
pub type MotorDriverTork = PB6<DefaultMode>;
pub type MotorDriverEnable = PA5<Output<PushPull>>;
pub type MotorDriverFault = PA1<Input<Floating>>;
pub type MotorAPwm = PA11<DefaultMode>;
pub type MotorBPwm = PA8<DefaultMode>;
pub type MotorAPhase = PA12<Output<PushPull>>;
pub type MotorBPhase = PA7<Output<PushPull>>;

// Buttons
pub type Wakeup = PA0<Input<Floating>>;

// SWD
pub type SwdIo = PA13<DefaultMode>;
pub type SwdClk = PA14<DefaultMode>;

pub struct Pins {
    // Qwiic I2C
    pub i2c_clk: I2cClk,
    pub i2c_sda: I2cSda,

    // Infared
    pub ir_tx: IrTx,
    pub ir_rx: IrRx,

    // Neopixel
    pub neopixel: Neopixel,

    // Power management
    pub standby: Standby,
    pub battery_sense: BatterySense,

    // Motor control
    pub motor_driver_tork: MotorDriverTork,
    pub motor_driver_enable: MotorDriverEnable,
    pub motor_driver_fault: MotorDriverFault,
    pub motor_a_pwm: MotorAPwm,
    pub motor_b_pwm: MotorBPwm,
    pub motor_a_phase: MotorAPhase,
    pub motor_b_phase: MotorBPhase,

    // Buttons
    pub wakeup_button: Wakeup,

    // SWD
    pub swd_io: SwdIo,
    pub swd_clk: SwdClk,
}

impl Pins {
    pub fn new(gpioa: GPIOA, gpiob: GPIOB, gpioc: GPIOC, rcc: &mut Rcc) -> Self {
        let port_a = gpioa.split(rcc);
        let port_b = gpiob.split(rcc);
        let port_c = gpioc.split(rcc);

        Self {
            // Qwiic I2C
            i2c_clk: port_b.pb8.into_open_drain_output_in_state(PinState::High),
            i2c_sda: port_b.pb9.into_open_drain_output_in_state(PinState::High),

            // Infared
            ir_tx: port_a.pa4,
            ir_rx: port_c.pc15.into(),

            // Neopixel
            neopixel: port_a.pa2,

            // Power management
            standby: port_a.pa6.into(),
            battery_sense: port_a.pa3,

            // Motor control
            motor_driver_tork: port_b.pb6,
            motor_driver_enable: port_a.pa5.into(),
            motor_driver_fault: port_a.pa1.into(),
            motor_a_pwm: port_a.pa11,
            motor_b_pwm: port_a.pa8,
            motor_a_phase: port_a.pa12.into(),
            motor_b_phase: port_a.pa7.into(),

            // SWD
            swd_io: port_a.pa13,
            swd_clk: port_a.pa14,

            // Buttons
            wakeup_button: port_a.pa0.into(),
        }
    }
}
