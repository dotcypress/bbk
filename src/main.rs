#![no_std]
#![no_main]
// #![deny(warnings)]

use defmt_rtt as _;
pub extern crate stm32g0xx_hal as hal;

extern crate panic_halt;
extern crate rtic;

mod esc;
mod ir;
mod pins;

use esc::*;
use hal::analog::adc;
use hal::gpio::SignalEdge;
use hal::i2c;
use hal::power::WakeUp;
use hal::prelude::*;
use hal::rcc::{self, PllConfig};
use hal::spi;
use hal::stm32;
use hal::stm32::*;
use ir::*;
use pins::*;
use smart_leds::*;
use ws2812_spi as ws2812;

#[rtic::app(device = stm32, peripherals = true)]
mod curio {
    use super::*;

    #[shared]
    struct Shared {
        ir: IrTransceiver,
        exti: EXTI,
        esc: MotorControl,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("init");
        // Configure APB bus clock to 48MHz, cause ws2812 requires 3Mbps SPI
        let pll_cfg = PllConfig::with_hsi(4, 24, 2);
        let rcc_cfg = rcc::Config::pll().pll_cfg(pll_cfg);
        let mut rcc = ctx.device.RCC.freeze(rcc_cfg);

        let pins = Pins::new(
            ctx.device.GPIOA,
            ctx.device.GPIOB,
            ctx.device.GPIOC,
            &mut rcc,
        );

        let mut exti = ctx.device.EXTI;
        let mut standby = pins.standby;
        let wakeup = pins.wakeup_button.listen(SignalEdge::Falling, &mut exti);

        let battery_sense = pins.battery_sense;
        let mut adc = ctx.device.ADC.constrain(&mut rcc);
        adc.set_sample_time(adc::SampleTime::T_80);
        adc.set_precision(adc::Precision::B_12);

        let mut ir_tim = ctx.device.TIM17.pwm(IR_TIMER_FREQUENCY.Hz(), &mut rcc);
        ir_tim.listen();

        let ir_carrier_tim = ctx.device.TIM14.pwm(IR_CARRIER_FREQUENCY.Hz(), &mut rcc);
        let rx = pins.ir_rx.listen(SignalEdge::All, &mut exti);
        let ir = IrTransceiver::new(ir_tim, ir_carrier_tim, pins.ir_tx, rx);

        let mut pwr = ctx.device.PWR.constrain(&mut rcc);
        pwr.clear_standby_flag();
        pwr.enable_wakeup_lane(WakeUp::Line1, SignalEdge::Falling);

        let i2c = ctx.device.I2C1.i2c(
            pins.i2c_sda,
            pins.i2c_clk,
            i2c::Config::new(400.kHz()),
            &mut rcc,
        );

        let spi = ctx.device.SPI1.spi(
            (spi::NoSck, spi::NoMiso, pins.neopixel),
            ws2812::MODE,
            3.MHz(),
            &mut rcc,
        );
        let mut neopixel = ws2812::Ws2812::new(spi);

        let mut data: [RGB<u8>; 10] = [RGB::default(); 10];
        for (idx, c) in data.iter_mut().enumerate() {
            *c = RGB {
                r: idx as _,
                g: 0,
                b: 0,
            };
        }
        neopixel.write(data.iter().cloned()).unwrap();

        standby.set_high().ok();
        let mut esc = MotorControl::new(
            ctx.device.TIM1,
            ctx.device.TIM16,
            pins.motor_driver_tork,
            pins.motor_driver_enable,
            pins.motor_driver_fault,
            pins.motor_a_pwm,
            pins.motor_b_pwm,
            pins.motor_a_phase,
            pins.motor_b_phase,
            &mut rcc,
        );

        esc.on();

        defmt::info!("init done");
        (Shared { ir, exti, esc }, Local {}, init::Monotonics())
    }

    #[task(binds = TIM1_BRK_UP_TRG_COM, shared = [esc])]
    fn esc_tick(ctx: esc_tick::Context) {
        let mut esc = ctx.shared.esc;
        esc.lock(|esc| esc.tick());
    }

    #[task(binds = EXTI0_1, shared = [exti])]
    fn wakeup(ctx: wakeup::Context) {
        let mut exti = ctx.shared.exti;
        exti.lock(|exti| exti.unpend(hal::exti::Event::GPIO0));
    }

    #[task(binds = TIM17, shared = [ir])]
    fn ir_timer_tick(ctx: ir_timer_tick::Context) {
        let mut ir = ctx.shared.ir;
        ir.lock(|ir| ir.tick());
    }

    #[task(binds = EXTI4_15, shared = [ir, exti])]
    fn ir_rx(ctx: ir_rx::Context) {
        let mut exti = ctx.shared.exti;
        let mut ir = ctx.shared.ir;
        exti.lock(|exti| exti.unpend(hal::exti::Event::GPIO15));
        if let Ok(Some(cmd)) = ir.lock(|ir| ir.event()) {
            defmt::info!("ir: {:?}", cmd);
        }
    }
}
