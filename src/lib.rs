#![no_std]
#![no_main]

#[cfg(feature = "defmt")]
use defmt::Format;

use bitfield_struct::bitfield;
use embedded_hal::i2c::I2c;

const MEAS1_MSB: u8 = 0x00; // MSB portion of Measurement 1
const MEAS1_LSB: u8 = 0x01; // LSB portion of Measurement 1
const MEAS2_MSB: u8 = 0x02; // MSB portion of Measurement 2
const MEAS2_LSB: u8 = 0x03; // LSB portion of Measurement 2
const MEAS3_MSB: u8 = 0x04; // MSB portion of Measurement 3
const MEAS3_LSB: u8 = 0x05; // LSB portion of Measurement 3
const MEAS4_MSB: u8 = 0x06; // MSB portion of Measurement 4
const MEAS4_LSB: u8 = 0x07; // LSB portion of Measurement 4
const FDC_CONF: u8 = 0x0C; // Capacitance to Digital Configuration
                           // const OFFSET_CAL_CIN1: u8 = 0x0D; //  CIN1 Offset Calibration
                           // const OFFSET_CAL_CIN2: u8 = 0x0E; //  CIN2 Offset Calibration
                           // const OFFSET_CAL_CIN3: u8 = 0x0F; //  CIN3 Offset Calibration
                           // const OFFSET_CAL_CIN4: u8 = 0x10; //  CIN4 Offset Calibration
                           // const GAIN_CAL_CIN1: u8 = 0x11; //  CIN1 Gain Calibration
                           // const GAIN_CAL_CIN2: u8 = 0x12; //  CIN2 Gain Calibration
                           // const GAIN_CAL_CIN3: u8 = 0x13; //  CIN3 Gain Calibration
                           // const GAIN_CAL_CIN4: u8 = 0x14; //  CIN4 Gain Calibration
const MANUFACTURER_ID: u8 = 0xFE; // 449 ID of Texas Instruments
const DEVICE_ID: u8 = 0xFF; // 004 ID of FDC1004 device
const ADDR: u8 = 80; // I2C device address

#[bitfield(u16, defmt = cfg(feature = "defmt"))]
pub struct MeasurementConfiguration {
    #[bits(5)]
    __: u8,
    #[bits(3)]
    pub channel_positive: CHANNEL,
    #[bits(3)]
    pub channel_negative: CHANNEL,
    #[bits(5)]
    pub offset_capacitance: u8,
}

#[repr(u8)]
pub enum ConfigureMeasurement {
    Measurement1 = 0x08,
    Measurement2 = 0x09,
    Measurement3 = 0x0A,
    Measurement4 = 0x0B,
}

#[repr(u8)]
#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(Format))]
pub enum CHANNEL {
    CIN1 = 0,
    CIN2 = 1,
    CIN3 = 2,
    CIN4 = 3,
}

impl CHANNEL {
    const fn into_bits(self) -> u16 {
        self as _
    }
    const fn from_bits(value: u16) -> Self {
        match value {
            0 => Self::CIN1,
            1 => Self::CIN2,
            2 => Self::CIN3,
            3 => Self::CIN4,
            _ => panic!("Invalid channel"),
        }
    }
}

#[bitfield(u16, defmt = cfg(feature = "defmt"))]
pub struct FDCConfiguration {
    #[bits(access = RO)]
    pub done_4: bool,
    #[bits(access = RO)]
    pub done_3: bool,
    #[bits(access = RO)]
    pub done_2: bool,
    #[bits(access = RO)]
    pub done_1: bool,
    pub meas_4: bool,
    pub meas_3: bool,
    pub meas_2: bool,
    pub meas_1: bool,
    pub repeat: bool,
    #[bits(1)]
    __: u8,
    #[bits(2)]
    pub rate: u8, // 1: 100 SPS, 2: 200 SPS, 3: 400 SPS

    #[bits(3)]
    __: u8,
    pub reset: bool,
}

pub struct FDC1004<T: I2c> {
    pub i2c: T,
}

impl<T: I2c> FDC1004<T> {
    pub fn get_configuration(&mut self) -> FDCConfiguration {
        let mut rx_buffer: [u8; 2] = [0; 2];
        self.i2c
            .write_read(ADDR, &[FDC_CONF], &mut rx_buffer)
            .unwrap();
        FDCConfiguration::from_bits(u16::from_be_bytes(rx_buffer))
    }

    pub fn set_configuration(&mut self, config: FDCConfiguration) {
        let [config_msb, config_lsb] = config.into_bits().to_be_bytes();
        self.i2c
            .write(ADDR, &[FDC_CONF, config_msb, config_lsb])
            .unwrap();
    }

    pub fn configure_measurement(
        &mut self,
        config: MeasurementConfiguration,
        measurement: ConfigureMeasurement,
    ) {
        let [config_msb, config_lsb] = config.into_bits().to_be_bytes();
        self.i2c
            .write(ADDR, &[measurement as u8, config_msb, config_lsb])
            .unwrap();
    }

    fn read_measurement(&mut self, meas_msb: u8, meas_lsb: u8) -> u32 {
        let mut buffer: [u8; 4] = [0; 4];
        self.i2c
            .write_read(ADDR, &[meas_msb], &mut buffer[0..2])
            .unwrap();
        self.i2c
            .write_read(ADDR, &[meas_lsb], &mut buffer[2..4])
            .unwrap();
        u32::from_be_bytes(buffer)
    }

    pub fn read_measurement_1(&mut self) -> u32 {
        self.read_measurement(MEAS1_MSB, MEAS1_LSB)
    }

    pub fn read_measurement_2(&mut self) -> u32 {
        self.read_measurement(MEAS2_MSB, MEAS2_LSB)
    }

    pub fn read_measurement_3(&mut self) -> u32 {
        self.read_measurement(MEAS3_MSB, MEAS3_LSB)
    }

    pub fn read_measurement_4(&mut self) -> u32 {
        self.read_measurement(MEAS4_MSB, MEAS4_LSB)
    }

    /// Read the manufacturer id. Should be 21577 (=0x5449)
    pub fn manufacturer_id(&mut self) -> u16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.i2c
            .write_read(ADDR, &[MANUFACTURER_ID], &mut buffer)
            .unwrap();
        u16::from_be_bytes(buffer)
    }

    /// Read the device id. Should be 4100 (=0x1004)
    pub fn device_id(&mut self) -> u16 {
        let mut buffer: [u8; 2] = [0; 2];
        self.i2c
            .write_read(ADDR, &[DEVICE_ID], &mut buffer)
            .unwrap();
        u16::from_be_bytes(buffer)
    }
}
