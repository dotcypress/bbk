#![no_std]

use hal::analog::adc;
use hal::gpio::SignalEdge;
use hal::power::WakeUp;
use hal::rcc::{self, PllConfig};
use hal::spi;
use hal::stm32::*;
use ws2812_spi as ws2812;

pub extern crate stm32g0xx_hal as hal;

mod ir;
mod motor;
mod pins;

pub use hal::i2c;
pub use hal::prelude::*;
pub use hal::stm32;
pub use infrared::*;
pub use ir::*;
pub use motor::*;
pub use pins::*;
pub use smart_leds::{SmartLedsWrite, RGB};

pub type SpiDev = spi::Spi<hal::pac::SPI1, (spi::NoSck, spi::NoMiso, Neopixel)>;
pub type I2cDev = hal::i2c::I2c<I2C1, I2cSda, I2cClk>;

pub struct Platform {
    pub adc: adc::Adc,
    pub exti: EXTI,
    pub battery_sense: BatterySense,
    pub i2c: I2cDev,
    pub ir: IrTransceiver,
    pub esc: MotorControl,
    pub neopixel: ws2812::Ws2812<SpiDev>,
    pub rcc: rcc::Rcc,
    pub standby: Standby,
    pub wakeup: Wakeup,
}

impl Platform {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rcc: RCC,
        pwr: PWR,
        exti: EXTI,
        adc: ADC,
        gpioa: GPIOA,
        gpiob: GPIOB,
        gpioc: GPIOC,
        tim1: TIM1,
        tim14: TIM14,
        tim16: TIM16,
        tim17: TIM17,
        spi: SPI1,
        i2c_dev: I2C1,
        i2c_config: i2c::Config,
    ) -> Self {
        // Configure APB bus clock to 48MHz, cause ws2812 requires 3Mbps SPI
        let pll_cfg = PllConfig::with_hsi(4, 24, 2);
        let rcc_cfg = rcc::Config::pll().pll_cfg(pll_cfg);
        let mut rcc = rcc.freeze(rcc_cfg);

        let pins = Pins::new(gpioa, gpiob, gpioc, &mut rcc);

        let mut exti = exti;
        let standby = pins.standby;
        let wakeup = pins.wakeup_button.listen(SignalEdge::Falling, &mut exti);

        let i2c = i2c_dev.i2c(pins.i2c_sda, pins.i2c_clk, i2c_config, &mut rcc);

        let spi = spi.spi(
            (spi::NoSck, spi::NoMiso, pins.neopixel),
            ws2812::MODE,
            3.MHz(),
            &mut rcc,
        );
        let neopixel = ws2812::Ws2812::new(spi);

        let battery_sense = pins.battery_sense;
        let mut adc = adc.constrain(&mut rcc);
        adc.set_sample_time(adc::SampleTime::T_80);
        adc.set_precision(adc::Precision::B_12);

        let mut ir_sample_tim = tim17.pwm(IR_SAMPLE_FREQUENCY.Hz(), &mut rcc);
        ir_sample_tim.listen();

        let ir_carrier_tim = tim14.pwm(IR_CARRIER_FREQUENCY.Hz(), &mut rcc);
        let ir = IrTransceiver::new(ir_sample_tim, ir_carrier_tim, pins.ir_tx, pins.ir_rx);

        let esc = MotorControl::new(
            tim1,
            tim16,
            pins.motor_driver_tork,
            pins.motor_driver_enable,
            pins.motor_driver_fault,
            pins.motor_a_pwm,
            pins.motor_b_pwm,
            pins.motor_a_phase,
            pins.motor_b_phase,
            &mut rcc,
        );

        let mut pwr = pwr.constrain(&mut rcc);
        pwr.clear_standby_flag();
        pwr.enable_wakeup_lane(WakeUp::Line1, SignalEdge::Falling);

        Self {
            adc,
            battery_sense,
            esc,
            exti,
            i2c,
            ir,
            neopixel,
            rcc,
            standby,
            wakeup,
        }
    }
}
