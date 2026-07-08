use std::num::NonZero;
use esp_idf_svc::hal::timer::config::{AlarmConfig, ClockSource, CountDirection, TimerConfig};
use esp_idf_svc::hal::timer::TimerDriver;
use esp_idf_svc::hal::units::Hertz;
use esp_idf_svc::sys::EspError;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use esp_idf_svc::hal::task::notification::Notifier;

pub struct Quantizer {
    pub ticks: Arc<AtomicU8>,
    pub steps: Arc<AtomicU8>,
    timer: TimerDriver<'static>
}

pub const PPQ: u8 = 96;
pub const TICKS_PER_STEP: u8 = PPQ / 4;
const TIMER_RESOLUTION: u32 = 40_000_000; // Hertz

impl Quantizer {
    pub fn new(notifier: Arc<Notifier>) -> Result<Self, EspError> {
        let ticks = Arc::new(AtomicU8::new(TICKS_PER_STEP - 1));
        let steps = Arc::new(AtomicU8::new(15));

        let mut timer_conf = TimerConfig::default();
        timer_conf.clock_source = ClockSource::Default;
        timer_conf.direction = CountDirection::Up;
        timer_conf.resolution = Hertz(TIMER_RESOLUTION);
        timer_conf.intr_priority = 3;

        log::info!("{:?}", timer_conf);

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
        let time = 60u64 * TIMER_RESOLUTION as u64;
        let ticks = bpm as u64 * PPQ as u64;
        let timer_step = time / ticks;
        log::info!("Timer Step: {timer_step}");

        self.timer.set_alarm_action(Some(&AlarmConfig {
            alarm_count: timer_step,
            auto_reload_on_alarm: true,
            reload_count: 0,
            ..Default::default()
        }))?;

        Ok(())
    }
}
