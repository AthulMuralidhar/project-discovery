#![no_main]
#![no_std]


mod serial_setup;
use cortex_m_rt::entry;
use core::fmt::Write;
use heapless::Vec;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};
use serial_setup::UartePort;

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

    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();
        loop {
            let byte = nb::block!(serial.read()).unwrap();

            if buffer.push(byte).is_err() {
                write!(serial, "error: buffer full\r\n").unwrap();
            }

            if byte == 13 {
                for byte in buffer.iter().rev().chain(&[b'\n', b'\r']) {
                    nb::block!(serial.write(*byte)).unwrap();
                }
                break;
            }

        }
        nb::block!(serial.flush()).unwrap()
    }

}