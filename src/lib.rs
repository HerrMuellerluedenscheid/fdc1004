#![no_std]
#![no_main]

#[cfg(feature="defmt")]
use defmt::info;

use embedded_hal::i2c::{I2c, Error};
use bitfield_struct::bitfield;

const MEAS1_MSB: u8 = 0x00;  // MSB portion of Measurement 1
const MEAS1_LSB: u8 = 0x01;  // LSB portion of Measurement 1
const MEAS2_MSB: u8 = 0x02;  // MSB portion of Measurement 2
const MEAS2_LSB: u8 = 0x03;  // LSB portion of Measurement 2
const MEAS3_MSB: u8 = 0x04;  // MSB portion of Measurement 3
const MEAS3_LSB: u8 = 0x05;  // LSB portion of Measurement 3
const MEAS4_MSB: u8 = 0x06;  // MSB portion of Measurement 4
const MEAS4_LSB: u8 = 0x07;  // LSB portion of Measurement 4
const CONF_MEAS1: u8 = 0x08;  // Measurement 1 Configuration
const CONF_MEAS2: u8 = 0x09;  // Measurement 2 Configuration
const CONF_MEAS3: u8 = 0x0A;  // Measurement 3 Configuration
const CONF_MEAS4: u8 = 0x0B;  // Measurement 4 Configuration
const FDC_CONF: u8 = 0x0C;  // Capacitance to Digital Configuration
const OFFSET_CAL_CIN1: u8 = 0x0D;  //  CIN1 Offset Calibration
const OFFSET_CAL_CIN2: u8 = 0x0E;  //  CIN2 Offset Calibration
const OFFSET_CAL_CIN3: u8 = 0x0F;  //  CIN3 Offset Calibration
const OFFSET_CAL_CIN4: u8 = 0x10;  //  CIN4 Offset Calibration
const GAIN_CAL_CIN1: u8 = 0x11;  //  CIN1 Gain Calibration
const GAIN_CAL_CIN2: u8 = 0x12;  //  CIN2 Gain Calibration
const GAIN_CAL_CIN3: u8 = 0x13;  //  CIN3 Gain Calibration
const GAIN_CAL_CIN4: u8 = 0x14;  //  CIN4 Gain Calibration
const MANUFACTURER_ID: u8 = 0xFE;  // 449 ID of Texas Instruments
const DEVICE_ID_TI: u8 = 0xFF;  // 004 ID of FDC1004 device
const ADDR: u8 = 80 ;  // I2C device address

#[bitfield(u16)]
pub struct FDCConfiguration {
    pub reset: bool,

    #[bits(3)]
    __: u8,

    #[bits(2)]
    pub rate: u8,

    #[bits(1)]
    __: u8,
    pub repeat: bool,
    pub meas_1: bool, 
    pub meas_2: bool, 
    pub meas_3: bool, 
    pub meas_4: bool, 
    pub done_1: bool, 
    pub done_2: bool, 
    pub done_3: bool, 
    pub done_4: bool, 
}

pub struct FDC1004<T: I2c> {
    pub i2c: T,
}

impl<T: I2c> FDC1004<T> {

    pub fn read_data(&mut self) {
        let mut rx_buffer: [u8; 2] = [0; 2];

        self.i2c.write_read(ADDR, &[DEVICE_ID_TI], &mut rx_buffer).unwrap();

        #[cfg(feature="defmt")]
        info!("{=[u8]:a}", rx_buffer);
    }

    pub fn get_configuration(&mut self) {
        let mut rx_buffer: [u8; 2] = [0; 2];
        self.i2c.write_read(ADDR, &[FDC_CONF], &mut rx_buffer).unwrap();

        #[cfg(feature="defmt")]
        info!("got configuration: {=[u8]:a}", rx_buffer);
    }

    pub fn set_configuration(&mut self, config: FDCConfiguration) {
        let mut rx_buffer: [u8; 4] = [0; 4];
    
        let [config_msb, config_lsb] = config.into_bits().to_be_bytes();
        self.i2c.write_read(ADDR, &[FDC_CONF, config_msb, config_lsb], &mut rx_buffer).unwrap();

        info!("{=[u8]:a}", rx_buffer);

    }
}
