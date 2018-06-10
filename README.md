# rust-i2c-tpa2016


[![Build status](https://img.shields.io/travis/querry43/rust-i2c-tpa2016.svg)](https://travis-ci.org/querry43/rust-i2c-tpa2016)
[![License](https://img.shields.io/github/license/querry43/rust-i2c-tpa2016.svg)](LICENSE)
[![crates.io](https://img.shields.io/crates/v/i2c-tpa2016.svg)](https://crates.io/crates/i2c-tpa2016)
[![Documentation](https://docs.rs/i2c-tpa2016/badge.svg)](https://docs.rs/i2c-tpa2016)

A rust crate for the communicating with the TPA2016 audio amplifier (https://www.adafruit.com/product/1712).

## Synopsys

```rust,no_run
extern crate i2cdev;
extern crate i2c_tpa2016;

use i2cdev::linux::LinuxI2CDevice;

fn main() {
    let i2cdev = LinuxI2CDevice::new("/dev/i2c-1", 0x58).unwrap();
    let mut tpa2016 = i2c_tpa2016::I2CTPA2016::new(i2cdev).unwrap();

    tpa2016.set_gain(10).unwrap();
    println!("gain = {:?}", tpa2016.get_gain().unwrap());

    tpa2016.enable_channel(true, true).unwrap();

    tpa2016.set_agc_compression(0).unwrap();

    tpa2016.set_release_control(0).unwrap();
    tpa2016.set_attack_control(0).unwrap();
    tpa2016.set_hold_control(0).unwrap();

    tpa2016.set_limit_level_on().unwrap();
    tpa2016.set_limit_level(10).unwrap();

    tpa2016.set_limit_level_off().unwrap();

    tpa2016.set_agc_max_gain(12).unwrap();
}
```
