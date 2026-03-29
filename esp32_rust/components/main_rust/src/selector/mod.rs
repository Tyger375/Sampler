use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use esp_idf_svc::hal::gpio::{AnyInputPin, Input, InterruptType, PinDriver, Pull};
use esp_idf_svc::hal::task::queue::Queue;
use esp_idf_svc::sys::{gpio_get_level, gpio_intr_enable};
use crate::get_time;

#[derive(Copy, Clone)]
pub enum RotationEvent {
    Left,
    Right
}

#[derive(Copy, Clone)]
pub enum SelectorEvent {
    Rotation(RotationEvent),
    Click(bool)
}

pub struct Selector<'a> {
    _clk: PinDriver<'a, Input>,
    _data: PinDriver<'a, Input>,
    _btn: PinDriver<'a, Input>,
    events_queue: Arc<Queue<SelectorEvent>>
}

impl<'a> Selector<'a> {
    pub fn new(
        clk_pin: AnyInputPin<'a>,
        data_pin: AnyInputPin<'a>,
        btn_pin: AnyInputPin<'a>
    ) -> Result<Self, anyhow::Error> {
        let mut clk = PinDriver::input(clk_pin, Pull::Floating)?;
        let data    = PinDriver::input(data_pin, Pull::Floating)?;
        let mut btn = PinDriver::input(btn_pin, Pull::Floating)?;

        let queue = Arc::new(Queue::<SelectorEvent>::new(10));

        unsafe {
            let clk_queue = queue.clone();
            let clk_raw = clk.pin();
            let data_raw = data.pin();

            let last_isr = AtomicU32::new(0);
            clk.subscribe(move || {
                let now = get_time();
                let last = last_isr.load(Ordering::Relaxed);

                if now.wrapping_sub(last) > 50_000 {
                    let clk_lvl = gpio_get_level(clk_raw as i32);
                    let data_lvl = gpio_get_level(data_raw as i32);

                    let event = if clk_lvl == data_lvl {
                        RotationEvent::Left
                    } else {
                        RotationEvent::Right
                    };
                    let event = SelectorEvent::Rotation(event);

                    clk_queue.send_back(event, 0).ok();
                    last_isr.store(now, Ordering::Relaxed);
                }

                gpio_intr_enable(clk_raw as i32);
            })?;

            let btn_queue = queue.clone();
            let btn_raw = btn.pin();

            let last_isr = AtomicU32::new(0);
            btn.subscribe(move || {
                let now = get_time();
                let last = last_isr.load(Ordering::Relaxed);

                if now.wrapping_sub(last) > 50_000 {
                    let btn_lvl = gpio_get_level(btn_raw as i32);

                    let event = SelectorEvent::Click(btn_lvl == 0);
                    btn_queue.send_back(event, 0).ok();
                    last_isr.store(now, Ordering::Relaxed);
                }

                gpio_intr_enable(btn_raw as i32);
            })?;
        }

        clk.set_interrupt_type(InterruptType::NegEdge)?;
        btn.set_interrupt_type(InterruptType::AnyEdge)?;

        clk.enable_interrupt()?;
        btn.enable_interrupt()?;

        Ok(
            Selector {
                _clk: clk,
                _data: data,
                _btn: btn,
                events_queue: queue
            }
        )
    }

    pub fn get_queue(&self) -> Arc<Queue<SelectorEvent>> {
        self.events_queue.clone()
    }
}
