
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;

use microbit::hal::prelude::*;
use microbit::{
    hal::twim, // twim is the i2c equivalent for this chip
    pac::twim0::frequency::FREQUENCY_A,
};

const ACCELEROMETER_ADDR: u8 = 0b0011001; // on the datasheet: 0011001b
const MAGNETOMETER_ADDR: u8 = 0b0011110; // on the datasheet: 0011110b

const ACCELEROMETER_REG_ID: u8 = 0x0f; // on the sheet: WHO_AM_I_A (0Fh) - h == hex
const MAGNETOMETER_REG_ID: u8 =  0x4f; // WHO_AM_I_M (4Fh)


#[entry]
fn main() -> ! {

    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut i2c = {
        twim::Twim::new(
            board.TWIM0,
            board.i2c_internal.into(),
            FREQUENCY_A::K100,
        )
    };

    let mut acc = [0];
    let mut mag = [0];

    // write the address and register
    i2c.write_read(ACCELEROMETER_ADDR, &[ACCELEROMETER_REG_ID], &mut acc).unwrap();
    i2c.write_read(MAGNETOMETER_ADDR, &[MAGNETOMETER_REG_ID], &mut mag).unwrap();

    // read the chip's responses
    rprintln!("the accelerometer chip is is:{:#b}", acc[0]);
    rprintln!("the magnetometer chip is is:{:#b}", mag[0]);


    loop {}
}