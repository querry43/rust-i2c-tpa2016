extern crate i2cdev;
#[macro_use] extern crate log;

use i2cdev::core::I2CDevice;
use std::cmp;

const TPA2016_SETUP: u8 = 0x1;
const TPA2016_SETUP_R_EN: u8 = 0x80;
const TPA2016_SETUP_L_EN: u8 = 0x40;

const TPA2016_ATK: u8 = 0x2;
const TPA2016_REL: u8 = 0x3;
const TPA2016_HOLD: u8 = 0x4;
const TPA2016_GAIN: u8 = 0x5;
const TPA2016_AGCLIMIT: u8 = 0x6;
const TPA2016_AGC: u8 = 0x7;

/// This object communicates with a PA2016 audio amplifier.
#[derive(Debug)]
pub struct I2CTPA2016<T: I2CDevice + Sized> {
    i2cdev: T,
}

impl<T> I2CTPA2016<T>
    where T: I2CDevice + Sized
{
    /// Constructs a new `I2CTPA2016<T>`.
    pub fn new(i2cdev: T) -> Result<I2CTPA2016<T>, T::Error> {
        Ok(I2CTPA2016 { i2cdev: i2cdev })
    }

    /// Set the gain from -28 and 30
    pub fn set_gain(&mut self, gain: i8) -> Result<(), T::Error> {
        debug!("set_gain({})", gain);
        let g = cmp::min(30, cmp::max(-28, gain));
        self.write(TPA2016_GAIN, g as u8)
    }

    /// Returns the gain value.
    pub fn get_gain(&mut self) -> Result<i8,  T::Error> {
        debug!("get_gain()");
        match self.read(TPA2016_GAIN) {
            Ok(gain) => {
                let mut g = gain as i8;
                if g & 0x20 > 0 {
                    #[allow(overflowing_literals)]
                    { g = g | 0xC0; }
                }
                Ok(g)
            },
            Err(e) => Err(e),
        }
    }

    pub fn enable_channel(&mut self, right: bool, left: bool) -> Result<(), T::Error> {
        debug!("enable_channel({:?}, {:?})", right, left);
        match self.read(TPA2016_SETUP) {
            Ok(mut setup) => {
                if right {
                    setup = setup | TPA2016_SETUP_R_EN;
                } else {
                    setup = setup & !TPA2016_SETUP_R_EN;
                }
                if left {
                    setup = setup | TPA2016_SETUP_L_EN;
                } else {
                    setup = setup & !TPA2016_SETUP_L_EN;
                }
                self.write(TPA2016_SETUP, setup)
            },
            Err(e) => Err(e),
        }
    }

    /// Set the automatic gain control compression.
    ///
    /// Values:
    ///   0 = 1:1
    ///   1 = 1:2
    ///   2 = 1:4
    ///   3 = 1:8
    pub fn set_agc_compression(&mut self, val: u8) -> Result<(), T::Error> {
        debug!("set_agc_compression({})", val);
        match self.read(TPA2016_AGC) {
            Ok(mut new_val) => {
                new_val = (new_val & 0xFC) | (val & 0x03);
                self.write(TPA2016_AGC, new_val)
            },
            Err(e) => Err(e),
        }
    }

    /// Set the release time in steps from 0 to 63 where each step is 0.0137s.
    pub fn set_release_control(&mut self, release: u8) -> Result<(), T::Error> {
        debug!("set_release_control({})", release);
        self.write(TPA2016_REL, release)
    }

    /// Set the attack time in steps from 0 to 63 where each step is 0.1067ms.
    pub fn set_attack_control(&mut self, attack: u8) -> Result<(), T::Error> {
        debug!("set_attack_control({})", attack);
        self.write(TPA2016_ATK, attack)
    }

    /// Set the hold time in steps from 0 to 63 where each step is 0.0137s.
    pub fn set_hold_control(&mut self, hold: u8) -> Result<(), T::Error> {
        debug!("set_hold_control({})", hold);
        self.write(TPA2016_HOLD, hold)
    }

    /// Enable level limiting.
    pub fn set_limit_level_on(&mut self) -> Result<(), T::Error> {
        debug!("set_limit_level_on()");
        match self.read(TPA2016_AGCLIMIT) {
            Ok(agc) => self.write(TPA2016_AGCLIMIT, agc & !0x80),
            Err(e) => Err(e),
        }
    }

    /// Disable level limiting.
    pub fn set_limit_level_off(&mut self) -> Result<(), T::Error> {
        debug!("set_limit_level_off()");
        match self.read(TPA2016_AGCLIMIT) {
            Ok(agc) => self.write(TPA2016_AGCLIMIT, agc | 0x80),
            Err(e) => Err(e),
        }
    }

    /// Set the level limit from 0 and 31.
    pub fn set_limit_level(&mut self, limit: u8) -> Result<(), T::Error> {
        debug!("set_limit_level({})", limit);
        match self.read(TPA2016_AGCLIMIT) {
            Ok(mut agc) => {
                agc = (agc & 0xE0) | (limit & 0x1F);
                self.write(TPA2016_AGCLIMIT, agc)
            },
            Err(e) => Err(e),
        }
    }

    /// Set the amx gain from 0 and 12.
    pub fn set_agc_max_gain(&mut self, max: u8) -> Result<(), T::Error> {
        debug!("set_agc_max_gain({})", max);
        match self.read(TPA2016_AGC) {
            Ok(mut agc) => {
                agc = (agc & 0x0F) | (max << 4);
                self.write(TPA2016_AGC, agc)
            },
            Err(e) => Err(e),
        }
    }

    fn write(&mut self, register: u8, data: u8) -> Result<(), T::Error> {
        trace!("Writing: {:?}", [register, data]);
        self.i2cdev.smbus_write_byte_data(register, data)
    }

    fn read(&mut self, register: u8) -> Result<u8, T::Error> {
        trace!("Reading from: {:?}", register);
        self.i2cdev.smbus_read_byte_data(register)
    }
}
