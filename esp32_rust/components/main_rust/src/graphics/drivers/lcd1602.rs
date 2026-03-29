use std::sync::{Arc, Mutex};
use esp_idf_svc::hal::i2c::I2cDriver;
use crate::delay_us;
use crate::graphics::drivers::GraphicsDriver;

const RS_BIT: u8 = 0x01;
const RW_BIT: u8 = 0x02;
const EN_BIT: u8 = 0x04;
const BL_BIT: u8 = 0x08;

pub struct Lcd1602 {
    i2c: Arc<Mutex<I2cDriver<'static>>>,
    addr: u8
}

impl Lcd1602 {
    pub fn new(
        i2c: Arc<Mutex<I2cDriver<'static>>>,
        addr: u8
    ) -> Box<dyn GraphicsDriver> {
        Box::new(
            Lcd1602 {
                i2c,
                addr
            }
        )
    }

    fn send_nibble(&self, nibble: u8, rs: bool) {
        let rs_bit = if rs {
            RS_BIT
        } else {
            0
        };
        let data: u8 = nibble | BL_BIT | rs_bit;

        {
            let mut guard = self.i2c.lock().unwrap();

            /* EN high */
            let d = data | EN_BIT;
            guard.write(self.addr, &[d], 100).unwrap();
            delay_us(1);

            /* EN low */
            let d = data;
            guard.write(self.addr, &[d], 100).unwrap();
            delay_us(40);
        }
    }

    fn send_command(&self, value: u8, rs: bool) {
        let high = value & 0xF0;
        let low = (value << 4) & 0xF0;

        self.send_nibble(high, rs);
        self.send_nibble(low, rs);
    }

    fn set_cursor(&self, col: u8, row: u8) {
        let col = if col > 15 {
            15
        } else {
            col
        };
        let row = if row > 1 {
            1
        } else {
            row
        };

        let address = if row == 0 {
            col
        } else {
            0x40 + col
        };
        self.send_command(0x80 | address, false);
    }

    fn write(&self, str: &String) {
        for c in str.as_bytes().iter() {
            self.send_command(*c, true);
        }
    }
}

impl GraphicsDriver for Lcd1602 {
    fn init(&mut self) {
        delay_us(50_000); // Wait for LCD power-up

        // Reset sequence (8-bit mode)
        self.send_nibble(0x30, false);
        delay_us(4500);

        self.send_nibble(0x30, false);
        delay_us(4500);

        self.send_nibble(0x30, false);
        delay_us(150);

        // Switch to 4-bit mode
        self.send_nibble(0x20, false);
        delay_us(150);

        // Function set: 4-bit, 2-line, 5x8 font
        self.send_command(0x28, false);

        // Display off
        self.send_command(0x08, false);

        // Clear display
        self.send_command(0x01, false);
        delay_us(2000);

        // Entry mode: increment, no shift
        self.send_command(0x06, false);

        // Display on, cursor off, blink off
        self.send_command(0x0C, false);

        log::info!(target: "lcd1602_driver", "Device initialized");
    }

    fn draw(&mut self, rows: &Vec<String>) {
        let mut i = 0;
        for row in rows.iter() {
            self.set_cursor(0, i);
            self.write(row);
            i += 1;
        }
    }

    fn clear(&mut self) {
        self.send_command(0x01, false);
        delay_us(2000);
    }
}
