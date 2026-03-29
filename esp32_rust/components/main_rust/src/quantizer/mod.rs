use std::num::NonZero;
use esp_idf_svc::hal::task::queue::Queue;
use esp_idf_svc::hal::timer::config::{AlarmConfig, ClockSource, CountDirection, TimerConfig};
use esp_idf_svc::hal::timer::TimerDriver;
use esp_idf_svc::hal::units::Hertz;
use esp_idf_svc::sys::EspError;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use esp_idf_svc::hal::task::notification::Notifier;

pub struct Quantizer<'a> {
    pub ticks: Arc<AtomicU8>,
    pub steps: Arc<AtomicU8>,
    timer: TimerDriver<'a>
}

const PPQ: u8 = 24;
const TICKS_PER_STEP: u8 = PPQ / 4;

impl<'a> Quantizer<'a> {
    pub fn new(notifier: Arc<Notifier>) -> Result<Self, EspError> {
        let ticks = Arc::new(AtomicU8::new(TICKS_PER_STEP - 1));
        let steps = Arc::new(AtomicU8::new(15));

        let mut timer_conf = TimerConfig::default();
        timer_conf.clock_source = ClockSource::Default;
        timer_conf.direction = CountDirection::Up;
        timer_conf.resolution = Hertz(40_000_000);
        timer_conf.intr_priority = 3;

        println!("{:?}", timer_conf);

        let mut timer = TimerDriver::new(&timer_conf)?;
        timer.subscribe_default()?;
        timer.enable()?;

        unsafe {
            let ticks = ticks.clone();
            let steps = steps.clone();

            timer.subscribe(move |_| {
                let t = ticks.fetch_add(1, Ordering::SeqCst);

                if t + 1 >= TICKS_PER_STEP {
                    ticks.store(0, Ordering::Relaxed);
                    let s = (steps.load(Ordering::SeqCst) + 1) % 16;
                    steps.store(s, Ordering::SeqCst);
                }

                notifier.notify_and_yield(NonZero::new(1).unwrap());
            })?;
        }

        timer.start()?;

        Ok(Quantizer {
            ticks,
            steps,
            timer
        })
    }

    pub fn start(&self, bpm: u8) -> Result<(), EspError> {
        //self.ticks.store(TICKS_PER_STEP - 1, Ordering::Relaxed);
        //self.steps.store(15, Ordering::Relaxed);

        let time = 60u64 * 40_000_000;
        let ticks = bpm as u64 * PPQ as u64;
        let timer_step = time / ticks;
        self.timer.set_alarm_action(Some(&AlarmConfig {
            alarm_count: timer_step,
            auto_reload_on_alarm: true,
            reload_count: 0,
            ..Default::default()
        }))?;

        Ok(())
    }
}
