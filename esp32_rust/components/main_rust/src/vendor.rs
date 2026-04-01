extern "C" {
    fn vendor_flush();
    fn vendor_out(packet: *const i8, length: usize);
}

pub struct Vendor;

impl Vendor {
    pub fn write(message: &str) {
        let bytes = message.as_bytes();
        let ptr = bytes.as_ptr() as *const i8;
        let len = bytes.len();

        unsafe {
            vendor_out(ptr, len);
        }
    }

    pub fn flush() {
        unsafe {
            vendor_flush();
        }
    }
}