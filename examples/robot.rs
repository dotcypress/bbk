#![no_std]
#![no_main]
#![deny(warnings)]

extern crate panic_halt;
extern crate rtic;

use defmt_rtt as _;

use bbk_bsp::*;

#[rtic::app(device = stm32, peripherals = true)]
mod curio {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        defmt::info!("init");

        let _platform = Platform::new(
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

        defmt::info!("init done");

        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            rtic::export::wfi();
        }
    }
}
