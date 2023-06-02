#![no_main]
#![no_std]

mod serial_setup;

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use serial_setup::UartePort;
use microbit::{
    hal::twim,
    pac::twim0::frequency::FREQUENCY_A,
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};
use lsm303agr::{AccelOutputDataRate, Lsm303agr};

// The challenge for this chapter is, to build a small application that communicates with the outside world via the serial interface introduced in the last chapter. It should be able to receive the commands "magnetometer" as well as "accelerometer" and then print the corresponding sensor data in response. This time no template code will be provided since all you need is already provided in the UART and this chapter. However, here are a few clues:
//
// You might be interested in core::str::from_utf8 to convert the bytes in the buffer to a &str, since we need to compare with "magnetometer" and "accelerometer".
// You will (obviously) have to read the documentation of the magnetometer API, however it's more or less equivalent to the accelerometer one

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    let i2c = {
        twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100)
    };

    // Code from crate documentation
    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz50).unwrap();

    loop {

        if sensor.accel_status().unwrap().xyz_new_data {
            let data = sensor.accel_data().unwrap();
            rprintln!("acceleration: x: {}, y: {}, z: {}", data.x, data.y, data.z);
            write!(serial, "acceleration: x: {}, y: {}, z: {}", data.x, data.y, data.z).unwrap();
        }
        nb::block!(serial.flush()).unwrap();

    }
}

