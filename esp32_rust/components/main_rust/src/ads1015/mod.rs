use esp_idf_svc::hal::i2c::I2cDriver;
use shared_bus::I2cProxy;

pub enum MuxConfig {
    DiffP0N1,
    DiffP0N3,
    DiffP1N3,
    DiffP2N3,
    MUX0,
    MUX1,
    MUX2,
    MUX3
}

impl Into<u8> for MuxConfig {
    fn into(&self) -> u8 {
        match self {
            MuxConfig::DiffP0N1 => 0,
            MuxConfig::DiffP0N3 => 1,
            MuxConfig::DiffP1N3 => 2,
            MuxConfig::DiffP2N3 => 3,
            MuxConfig::MUX0 => 4,
            MuxConfig::MUX1 => 5,
            MuxConfig::MUX2 => 6,
            MuxConfig::MUX3 => 7
        }
    }
}

pub enum FsrConfig {
    Fsr6_144,
    Fsr4_096,
    Fsr2_048,
    Fsr1_024,
    Fsr0_512,
    Fsr0_256,
}

impl Into<u8> for FsrConfig {
    fn into(&self) -> u8 {
        match self {
            FsrConfig::Fsr6_144 => 0,
            FsrConfig::Fsr4_096 => 1,
            FsrConfig::Fsr2_048 => 2,
            FsrConfig::Fsr1_024 => 3,
            FsrConfig::Fsr0_512 => 4,
            FsrConfig::Fsr0_256 => 5,
        }
    }
}

pub enum OpModeConfig {
    Continuous,
    SingleShot,
}

impl Impl<u8> for OpModeConfig {
    fn into(&self) -> u8 {
        match self {
            OpModeConfig::Continuous => 0,
            OpModeConfig::SingleShot => 1
        }
    }
}

pub enum DataRateConfig {
    DataRate128,
    DataRate250,
    DataRate490,
    DataRate920,
    DataRate1600,
    DataRate2400,
    DataRate3300,
}

impl Into<u8> for DataRateConfig {
    fn into(&self) -> u8 {
        match self {
            DataRateConfig::DataRate128 => 0,
            DataRateConfig::DataRate250 => 1,
            DataRateConfig::DataRate490 => 2,
            DataRateConfig::DataRate920 => 3,
            DataRateConfig::DataRate1600 => 4,
            DataRateConfig::DataRate2400 => 5,
            DataRateConfig::DataRate3300 => 6,
        }
    }
}

pub enum CompModeConfig {
    Traditional,
    Window
}

impl Into<u8> for CompModeConfig {
    fn into(&self) -> u8 {
        match self {
            CompModeConfig::Traditional => 0,
            CompModeConfig::Window => 1
        }
    }
}

pub enum CompPolarityConfig {
    ActiveLow,
    ACtiveHigh
}

impl Into<u8> for CompPolarityConfig {
    fn into(&self) -> u8 {
        match self {
            CompPolarityConfig::ActiveLow => 0,
            CompPolarityConfig::ACtiveHigh => 1
        }
    }
}

pub enum CompLatchingConfig {
    NonLatching,
    Latching
}

impl Into<u8> for CompLatchingConfig {
    fn into(&self) -> u8 {
        match self {
            CompLatchingConfig::NonLatching => 0,
            CompLatchingConfig::Latching => 1
        }
    }
}

pub enum CompQueueDisableConfig {
    AssertAfterOne,
    AssertAfterTwo,
    AssertAfterFour,
    DisableComp
}

impl Into<u8> for CompQueueDisableConfig {
    fn into(&self) -> u8 {
        match self {
            CompQueueDisableConfig::AssertAfterOne => 0,
            CompQueueDisableConfig::AssertAfterTwo => 1,
            CompQueueDisableConfig::AssertAfterFour => 2,
            CompQueueDisableConfig::DisableComp => 3,
        }
    }
}

pub struct ADS1015Config {
    mux_config: MuxConfig,
    fsr_mode: FSRConfig,
    op_mode: OP_Mode_Config,
    data_rate: Data_Rate_Config,
    comparator_mode: Comp_Mode_Config,
    comparator_polarity: Comp_Polarity_Config,
    comparator_latching: Comp_Latching_Config,
    queue_and_disable: Comp_Queue_Disable_Config
}

pub struct ADS1015<'a> {
    i2c: I2cProxy,
    cfg_reg: u16,
    addr: u8
}

impl<'a> ADS1015<'a> {
    pub fn new(
        i2c: I2cProxy,
        addr: u8
    ) -> Self {
        ADS1015 {
            i2c,
            cfg_reg: 0,
            addr
        }
    }

    pub fn set_config_reg(&self) {
        const reg_num: u8 = 0x1;
        // master transmit
    }

    pub fn set_config(&mut self, config: &ADS1015Config) {
        let bytes: [u8; 3] = [0x01, 0, 0];

        let mux_config: u8 = config.mux_config.into();
        let fsr_mode: u8 = config.fsr_mode.into();
        let op_mode: u8 = config.op_mode.into();

        let data_rate: u8 = config.data_rate.into();
        let comparator_mode: u8 = config.comparator_mode.into();
        let comparator_polarity: u8 = config.comparator_polarity.into();
        let comparator_latching: u8 = config.comparator_latching.into();
        let queue_and_disable: u8 = config.queue_and_disable.into();

        bytes[1] = ((mux_config & 0b111) << 4) | ((fsr_mode & 0b111) << 1) | ((op_mode & 0b1) << 0);
        bytes[2] = ((data_rate & 0b111) << 5) | ((comparator_mode & 0b1) << 4) | ((comparator_polarity & 0b1) << 3) | ((comparator_latching & 0b1) << 2) | ((queue_and_disable & 0b11) << 0);

        // master transmit
        let err = false;

        if !err {
            log::info!("Configuration updated!");
            self.cfg_reg = bytes[1] << 8 | bytes[2];
        }

        
    }
}