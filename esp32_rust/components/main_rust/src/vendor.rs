use std::cmp::min;
use std::thread;
use std::time::Duration;

extern "C" {
    fn vendor_flush();
    fn vendor_out(packet: *const i8, length: usize);
    fn vendor_in(packet: *mut i8, length: usize) -> u32;
    fn vendor_write_available() -> u32;
    fn vendor_available() -> u32;
}

pub struct Vendor;

impl Vendor {
    pub fn write_raw(message: &str) {
        let bytes = message.as_bytes();
        let ptr = bytes.as_ptr() as *const i8;
        let len = bytes.len();

        unsafe {
            vendor_out(ptr, len);
        }
    }

    pub fn respond(mut message: String) {
        message.push('\n');

        let mut data = message.as_str();

        while !data.is_empty() {
            let available = Self::write_available() as usize;

            if available > 0 {
                let to_send = min(data.len(), available);
                Self::write_raw(&data[..to_send]);

                data = &data[to_send..];
                Self::flush();
            } else {
                thread::sleep(Duration::from_millis(1));
            }
        }
    }

    pub fn read(buffer_size: usize) -> String {
        let mut buffer = vec![0u8; buffer_size];

        unsafe {
            let bytes_read = vendor_in(buffer.as_mut_ptr() as *mut i8, buffer_size) as usize;
            buffer.set_len(bytes_read);
        }

        String::from_utf8_lossy(&buffer).into_owned()
    }

    pub fn flush() {
        unsafe {
            vendor_flush();
        }
    }

    pub fn write_available() -> u32 {
        unsafe {
            vendor_write_available()
        }
    }

    pub fn available() -> u32 {
        unsafe {
            vendor_available()
        }
    }
}