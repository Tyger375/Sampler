use esp_idf_svc::hal::i2c::I2cDriver;

pub struct ADS1015<'a> {
    i2c_master: &'a mut I2cDriver<'a>,
    cfgReg: u16,
    addr: u8
}

impl<'a> ADS1015<'a> {
    pub(crate) fn new(
        i2c_master: &'a mut I2cDriver<'a>,
        addr: u8
    ) -> Self {
        ADS1015 {
            i2c_master,
            cfgReg: 0,
            addr
        }
    }
}