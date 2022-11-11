#![no_std]
#![no_main]
// #![deny(warnings)]

use defmt_rtt as _;
pub extern crate stm32g0xx_hal as hal;

extern crate panic_halt;
extern crate rtic;

mod ir;
mod esc;
mod pins;
mod platform;

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
use esc::*;
use pins::*;
use platform::*;
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

        let mut platform = Platform::new(
            ctx.device.RCC,
            ctx.device.PWR,
            ctx.device.EXTI,
            ctx.device.ADC,
            ctx.device.GPIOA,
            ctx.device.GPIOB,
            ctx.device.GPIOC,
            ctx.device.TIM1,
            ctx.device.TIM14,
            ctx.device.TIM16,
            ctx.device.TIM17,
            ctx.device.SPI1,
            ctx.device.I2C1,
            i2c::Config::new(400.kHz()),
        );

        platform.standby.set_high().ok();

        platform.esc.enable.set_high().ok();
        platform.esc.tork.set_duty(4000);
        // platform
        //     .esc
        //     .pwm_a
        //     .set_duty(platform.esc.pwm_a.get_max_duty() / 2);
        // platform
        //     .esc
        //     .pwm_b
        //     .set_duty(platform.esc.pwm_b.get_max_duty() / 2);

        let mut delay = ctx.core.SYST.delay(&mut platform.rcc);
        let mut cnt: usize = 0;
        let mut data: [RGB<u8>; 10] = [RGB::default(); 10];
        loop {
            for (idx, c) in data.iter_mut().enumerate() {
                *c = RGB {
                    r: 24 - ((cnt + idx) % 24) as u8,
                    g: 0, //(255 - cnt) as u8,
                    b: 0, //((cnt + idx) % 24) as u8,
                };
            }
            platform.neopixel.write(data.iter().cloned()).unwrap();
            cnt += 1;
            delay.delay(20.millis());
        }

        defmt::info!("init done");

        (
            Shared {
                ir: platform.ir,
                exti: platform.exti,
                esc: platform.esc,
            },
            Local {},
            init::Monotonics(),
        )
    }

    #[task(binds = TIM1_BRK_UP_TRG_COM, shared = [])]
    fn esc_tick(_ctx: esc_tick::Context) {
        defmt::info!("esc");
    }

    #[task(binds = EXTI0_1, shared = [exti])]
    fn wakeup(ctx: wakeup::Context) {
        let mut exti = ctx.shared.exti;
        exti.lock(|exti| exti.unpend(hal::exti::Event::GPIO0));
        defmt::info!("wakeup");
    }

    #[task(binds = TIM17, shared = [ir])]
    fn ir_timer_tick(ctx: ir_timer_tick::Context) {
        let mut ir = ctx.shared.ir;
        ir.lock(|ir| ir.tick());
    }

    #[task(binds = EXTI4_15, shared = [ir, exti, esc])]
    fn ir_rx(ctx: ir_rx::Context) {
        let mut exti = ctx.shared.exti;
        let mut ir = ctx.shared.ir;
        let mut esc = ctx.shared.esc;
        exti.lock(|exti| exti.unpend(hal::exti::Event::GPIO15));
        if let Ok(Some(cmd)) = ir.lock(|ir| ir.event()) {
            defmt::info!("ir: {:?}", cmd);
            esc.lock(|esc| match cmd.cmd {
                64 => {
                    esc.pwm_a.set_duty(
                        esc.pwm_a
                            .get_duty()
                            .saturating_add(2048)
                            .min(esc.pwm_a.get_max_duty()),
                    );
                    esc.pwm_b.set_duty(
                        esc.pwm_b
                            .get_duty()
                            .saturating_add(2048)
                            .min(esc.pwm_b.get_max_duty()),
                    );
                }
                25 => {
                    esc.pwm_a
                        .set_duty(esc.pwm_a.get_duty().saturating_sub(2048));
                    esc.pwm_b
                        .set_duty(esc.pwm_b.get_duty().saturating_sub(2048));
                }
                7 => {
                    esc.pwm_a
                        .set_duty(esc.pwm_a.get_duty().saturating_sub(2048));

                    esc.pwm_b.set_duty(
                        esc.pwm_b
                            .get_duty()
                            .saturating_add(2048)
                            .min(esc.pwm_b.get_max_duty()),
                    );
                }
                9 => {
                    esc.pwm_a.set_duty(
                        esc.pwm_a
                            .get_duty()
                            .saturating_add(2048)
                            .min(esc.pwm_a.get_max_duty()),
                    );
                    esc.pwm_b
                        .set_duty(esc.pwm_b.get_duty().saturating_sub(2048));
                }
                21 => {
                    let duty = esc.pwm_a.get_duty() / 2 + esc.pwm_b.get_duty() / 2;
                    esc.pwm_a.set_duty(duty);
                    esc.pwm_b.set_duty(duty);
                }
                70 => {
                    esc.pwm_a.set_duty(0);
                    esc.pwm_b.set_duty(0);
                }
                _ => {}
            })
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            rtic::export::nop();
        }
    }
}
