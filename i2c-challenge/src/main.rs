#![no_main]
#![no_std]

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
use microbit::hal::prelude::*;
use lsm303agr::{AccelOutputDataRate, MagOutputDataRate, Lsm303agr};
use heapless::Vec;
use nb::block;
use core::fmt::Write;
mod serial_setup;
use core::str;

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
    sensor.set_mag_odr(MagOutputDataRate::Hz50).unwrap();

    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    loop {
        let mut buffer: Vec<u8, 32> = Vec::new();

        loop {
            let byte = block!(serial.read()).unwrap();
            
            if byte == 13 {
                break;
            }

            if buffer.push(byte).is_err() {
                write!(serial, "error: buffer full\r\n").unwrap();
                break;
            }
        }

        if str::from_utf8(&buffer).unwrap().trim() == "accelerometer" {
            while !sensor.accel_status().unwrap().xyz_new_data {}

            let data = sensor.accel_data().unwrap();
            write!(serial, "Accelerometer: x {} y {} z {}\r\n", data.x, data.y, data.z).unwrap();
        } else if str::from_utf8(&buffer).unwrap().trim() == "magnetometer" {
            while !sensor.mag_status().unwrap().xyz_new_data {}

            let data = sensor.mag_data().unwrap();
            write!(serial, "Magnetometer: x {} y {} z {}\r\n", data.x, data.y, data.z).unwrap();
        } else {
            write!(serial, "error: command not detected\r\n").unwrap();
        }

    }
}

