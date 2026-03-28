use anyhow::anyhow;
use esp_idf_svc::hal::i2c::I2cDriver;
use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::{Arc, Mutex};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MuxConfig {
    DiffP0N1 = 0,
    DiffP0N3 = 1,
    DiffP1N3 = 2,
    DiffP2N3 = 3,
    MUX0 = 4,
    MUX1 = 5,
    MUX2 = 6,
    MUX3 = 7,
}

impl MuxConfig {
    pub fn mux_from(value: u8) -> Self {
        match value {
            0 => MuxConfig::MUX0,
            1 => MuxConfig::MUX1,
            2 => MuxConfig::MUX2,
            3 => MuxConfig::MUX3,
            _ => MuxConfig::MUX0,
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsrConfig {
    Fsr6_144 = 0,
    Fsr4_096 = 1,
    Fsr2_048 = 2,
    Fsr1_024 = 3,
    Fsr0_512 = 4,
    Fsr0_256 = 5,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpModeConfig {
    Continuous = 0,
    SingleShot = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataRateConfig {
    DataRate128 = 0,
    DataRate250 = 1,
    DataRate490 = 2,
    DataRate920 = 3,
    DataRate1600 = 4,
    DataRate2400 = 5,
    DataRate3300 = 6,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompModeConfig {
    Traditional = 0,
    Window = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompPolarityConfig {
    ActiveLow = 0,
    ActiveHigh = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompLatchingConfig {
    NonLatching = 0,
    Latching = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompQueueDisableConfig {
    AssertAfterOne = 0,
    AssertAfterTwo = 1,
    AssertAfterFour = 2,
    DisableComp = 3,
}

pub struct ADS1015Config {
    pub mux_config: MuxConfig,
    pub fsr_mode: FsrConfig,
    pub op_mode: OpModeConfig,
    pub data_rate: DataRateConfig,
    pub comparator_mode: CompModeConfig,
    pub comparator_polarity: CompPolarityConfig,
    pub comparator_latching: CompLatchingConfig,
    pub queue_and_disable: CompQueueDisableConfig,
}

pub struct ADS1015 {
    i2c: Arc<Mutex<I2cDriver<'static>>>,
    cfg_reg: AtomicU16,
    addr: u8,
}

impl ADS1015 {
    pub fn new(i2c: Arc<Mutex<I2cDriver<'static>>>, addr: u8) -> Self {
        ADS1015 {
            i2c,
            cfg_reg: AtomicU16::new(0),
            addr,
        }
    }

    pub fn set_config_reg(&self) -> Result<(), anyhow::Error> {
        const REG_NUM: [u8; 1] = [0x1];
        let mut guard = self.i2c.lock().map_err(|_| anyhow!(""))?;
        guard.write(self.addr, &REG_NUM, 1000)?;

        Ok(())
    }

    pub fn set_mux(&self, mux: &MuxConfig) -> Result<(), anyhow::Error> {
        let mut bytes: [u8; 3] = [0x01, 0, 0];
        const MASK: u8 = !0b0111_0000u8;

        let cfg_reg = self.cfg_reg.load(Ordering::Relaxed);

        bytes[1] = ((cfg_reg >> 8) & 0xFF) as u8;
        bytes[1] &= MASK;
        bytes[1] |= (*mux as u8 & 0b111) << 4;

        bytes[2] = (cfg_reg & 0xFF) as u8;

        let mut guard = self.i2c.lock().map_err(|_| anyhow!(""))?;
        if let Ok(_) = guard.write(self.addr, &bytes, 1000) {
            let value = ((bytes[1] as u16) << 8) | (bytes[2] as u16);
            self.cfg_reg.store(value, Ordering::Relaxed);
            Ok(())
        } else {
            Err(anyhow!(""))
        }
    }

    pub fn set_config(&self, config: &ADS1015Config) -> Result<(), anyhow::Error> {
        let mut bytes: [u8; 3] = [0x01, 0, 0];

        let mux_config = config.mux_config as u8;
        let fsr_mode = config.fsr_mode as u8;
        let op_mode = config.op_mode as u8;

        let data_rate = config.data_rate as u8;
        let comparator_mode = config.comparator_mode as u8;
        let comparator_polarity = config.comparator_polarity as u8;
        let comparator_latching = config.comparator_latching as u8;
        let queue_and_disable = config.queue_and_disable as u8;

        bytes[1] = ((mux_config & 0b111) << 4) | ((fsr_mode & 0b111) << 1) | ((op_mode & 0b1) << 0);
        bytes[2] = ((data_rate & 0b111) << 5)
            | ((comparator_mode & 0b1) << 4)
            | ((comparator_polarity & 0b1) << 3)
            | ((comparator_latching & 0b1) << 2)
            | ((queue_and_disable & 0b11) << 0);

        let mut guard = self.i2c.lock().map_err(|_| anyhow!(""))?;
        if let Ok(_) = guard.write(self.addr, &bytes, 1000) {
            log::info!("Configuration updated!");
            let value = ((bytes[1] as u16 & 0xFF) << 8) | (bytes[2] as u16 & 0xFF);
            self.cfg_reg.store(value, Ordering::Relaxed);
            Ok(())
        } else {
            Err(anyhow!(""))
        }
    }

    pub fn read_config(&self) -> Result<(), anyhow::Error> {
        self.set_config_reg()?;

        let mut bytes: [u8; 2] = [0, 0];
        {
            let mut guard = self.i2c.lock().map_err(|_| anyhow!(""))?;
            guard.read(self.addr, &mut bytes, 1000)?;
        }

        let value = ((bytes[0] as u16 & 0xFF) << 8) | (bytes[1] as u16 & 0xFF);
        self.cfg_reg.store(value, Ordering::Relaxed);

        log::info!("Read config: {:b}", value);

        Ok(())
    }

    pub fn read(&self) -> u16 {
        const REG_ADDR: [u8; 1] = [0x0];
        let mut data: [u8; 2] = [0, 0];

        {
            if let Ok(mut guard) = self.i2c.lock() {
                guard
                    .write_read(self.addr, &REG_ADDR, &mut data, 1000)
                    .unwrap();
            }
        }

        let raw_val = ((data[0] as u16) << 8) | (data[1] as u16);
        raw_val >> 4
    }
}
