use std::ptr;
use esp_idf_svc::sys::uxTaskGetStackHighWaterMark;

extern "C" {
    fn esp_delay_us(micros: u32);
    fn log_timestamp() -> u32;
    fn timer_get_time() -> u32;
}

pub fn delay_us(micros: u32) {
    unsafe {
        esp_delay_us(micros);
    }
}

pub fn timestamp() -> u32 {
    unsafe { log_timestamp() }
}

pub fn get_time() -> u32 {
    unsafe { timer_get_time() }
}

pub fn log_main_stack(label: &str) {
    // Get the minimum free stack space ever reached by the current task
    // Returns value in "words" (4 bytes each)
    let stack_words = unsafe { uxTaskGetStackHighWaterMark(ptr::null_mut()) };
    let stack_bytes = stack_words * 4;

    log::info!("--- Stack Report [{}] ---", label);
    log::info!("Min free stack: {} bytes", stack_bytes);
}