use crate::*;
use core::convert::Infallible;
use hal::timer::pwm::{Pwm, PwmPin};
use hal::timer::Channel1;
use infrared::protocol::nec::NecCommand;
use infrared::{protocol::*, Receiver};
use infrared::receiver::Error;
use infrared::sender::Sender;

pub const IR_TIMER_FREQUENCY: u32 = 100_000;
pub const IR_CARRIER_FREQUENCY: u32 = 38_000;

pub struct IrTransceiver {
    ir_tim: Pwm<TIM17>,
    tx: Sender<PwmPin<TIM14, Channel1>, { IR_TIMER_FREQUENCY }, 64>,
    rx: Receiver<Nec, IrRx>,
    ts: u32,
    event_ts: u32,
}

impl IrTransceiver {
    pub fn new(ir_tim: Pwm<TIM17>, carrier_tim: Pwm<TIM14>, tx_pin: IrTx, rx_pin: IrRx) -> Self {
        let mut tx_pin = carrier_tim.bind_pin(tx_pin);
        tx_pin.set_duty(tx_pin.get_max_duty() / 2);

        let tx = infrared::sender::Sender::new(tx_pin);
        let rx = infrared::receiver::Receiver::with_input(IR_TIMER_FREQUENCY, rx_pin);

        Self {
            ir_tim,
            tx,
            rx,
            ts: 0,
            event_ts: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tx.tick();
        self.ts = self.ts.wrapping_add(1);
        self.ir_tim.clear_irq();
    }

    pub fn event(&mut self) -> Result<Option<NecCommand>, Error<Infallible>> {
        let dt = self.ts.wrapping_sub(self.event_ts);
        self.event_ts = self.ts;
        self.rx.event(dt)
    }

    pub fn send(&mut self, cmd: &NecCommand) {
        self.tx.load::<Nec>(cmd);
    }
}
