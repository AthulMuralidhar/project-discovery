#![no_std]
#![no_main]

use cortex_m_rt::entry;
use rtt_target::rtt_init_print;
use panic_rtt_target as _;
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};
mod serial_setup;
use serial_setup::UartePort;
use microbit::Board;

// What is UARTE?
// The microcontroller has a peripheral called UART, which stands for Universal Asynchronous Receiver/Transmitter

// before flashing it, i need to run minicom - the cmd is here:
// sudo minicom -D /dev/ttyACM0 -b 115200

#[entry]
fn main() -> !{
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    nb::block!(serial.write(b'x')).unwrap();
    nb::block!(serial.flush()).unwrap();

    loop {}
}
