use esp_idf_svc::hal::task::queue::Queue;
use esp_idf_svc::hal::timer::config::{AlarmConfig, TimerConfig};
use esp_idf_svc::hal::timer::TimerDriver;
use esp_idf_svc::hal::units::Hertz;
use esp_idf_svc::sys::EspError;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct Quantizer<'a> {
    ticks: Arc<AtomicU8>,
    steps: Arc<AtomicU8>,
    timer: TimerDriver<'a>,
    queue: Arc<Queue<(u8, u8)>>
}

const PPQ: u8 = 24;
const TICKS_PER_STEP: u8 = PPQ / 4;

impl<'a> Quantizer<'a> {
    pub fn new() -> Result<Self, EspError> {
        let queue = Arc::new(Queue::new(64));
        let ticks = Arc::new(AtomicU8::new(TICKS_PER_STEP - 1));
        let steps = Arc::new(AtomicU8::new(15));

        let mut timer_conf = TimerConfig::default();
        timer_conf.resolution = Hertz(1_000_000);
        timer_conf.intr_priority = 3;

        println!("{:?}", timer_conf);

        let mut timer = TimerDriver::new(&timer_conf)?;
        timer.subscribe_default()?;
        timer.enable()?;

        {
            let queue = queue.clone();
            let ticks = ticks.clone();
            let steps = steps.clone();

            timer.subscribe(move |_| {
                let update_result = ticks.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |t| {
                    let next = t + 1;
                    if next >= TICKS_PER_STEP {
                        Some(0)
                    } else {
                        Some(next)
                    }
                });

                if let Ok(old_ticks) = update_result {
                    let current_ticks = if old_ticks + 1 >= TICKS_PER_STEP {
                        0
                    } else {
                        old_ticks + 1
                    };
                    if current_ticks == 0 {
                        steps.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |s| {
                            Some((s + 1) % 16)
                        }).unwrap();
                    }

                    let current_steps = steps.load(Ordering::Relaxed);
                    queue.send_back((current_ticks, current_steps), 0).unwrap();
                }
            })?;
        }

        Ok(Quantizer {
            ticks,
            steps,
            timer,
            queue
        })
    }

    pub fn start(&mut self, bpm: u8) -> Result<(), EspError> {
        self.ticks.store(TICKS_PER_STEP - 1, Ordering::Relaxed);
        self.steps.store(15, Ordering::Relaxed);

        let timer_step = (60 * 1_000_000) / (bpm as u64 * PPQ as u64);
        self.timer.set_alarm_action(Some(&AlarmConfig {
            alarm_count: self
                .timer
                .duration_to_count(Duration::from_micros(timer_step))?,
            auto_reload_on_alarm: true,
            ..Default::default()
        }))?;

        self.timer.start()?;

        Ok(())
    }

    pub fn get_queue(&mut self) -> Arc<Queue<(u8, u8)>> {
        self.queue.clone()
    }
}
