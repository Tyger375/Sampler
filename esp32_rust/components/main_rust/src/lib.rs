use std::ffi::CStr;
use std::thread;
use std::time::Duration;
use esp_idf_svc::hal::cpu::Core;
use esp_idf_svc::hal::delay;
use core::default::Default;
use esp_idf_svc::hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::hal::task::thread::ThreadSpawnConfiguration;
use esp_idf_svc::hal::units::Hertz;
use crate::ads1015::ADS1015;
use crate::midi::MIDI;
use crate::quantizer::{Quantizer};
use shared_bus;

pub mod midi;
pub mod quantizer;
pub mod ads1015;

#[no_mangle]
extern "C" fn rust_main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let i2c_config = I2cConfig::new().baudrate(Hertz(400_000));
    let i2c_master = I2cDriver::new(peripherals.i2c0, peripherals.pins.gpio12, peripherals.pins.gpio11, &i2c_config).unwrap();

    //let ads1 = ADS1015::new(&mut i2c_master, 0x48);
    //let ads2 = ADS1015::new(&mut i2c_master, 0x49);

    let mut quantizer = Quantizer::new().expect("Failed to create quantizer");
    quantizer.start(140).unwrap();

    ThreadSpawnConfiguration {
        name: Some(CStr::from_bytes_with_nul(b"quantizer_task\0").unwrap()),
        stack_size: 4096,
        priority: 24,
        pin_to_core: Some(Core::Core1),
        ..Default::default()
    }.set().unwrap();
    let quantizer_queue = quantizer.get_queue();
    thread::spawn(move || {
        loop {
            // Quantizer ticks
            if let Some((_packet, _)) = quantizer_queue.recv_front(delay::BLOCK) {
                let sync_packet = [0x0F, 0xF8, 0x00, 0x00];
                MIDI::send(&sync_packet);

                // handle sequencer
            }
        }
    });
    ThreadSpawnConfiguration::default().set().unwrap();

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
