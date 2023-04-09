#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::board::Board;
use microbit::hal::timer::Timer;
use microbit::hal::prelude::*;

#[entry]
fn main() -> ! {

rtt_init_print!();

let mut board = Board::take().unwrap();

// board.TIMER0 is a peripheral

// what is a peripheral?
// Most Microcontrollers have more than just a CPU, RAM, or Flash Memory - they contain sections of silicon which are used for interacting with systems outside of the microcontroller, as well as directly and indirectly interacting with their surroundings in the world via sensors, motor controllers, or human interfaces such as a display or keyboard. These components are collectively known as Peripherals.

// there is also a more detailed description in the specsheet here: https://infocenter.nordicsemi.com/pdf/nRF51_RM_v3.0.1.pdf
// at page 99 - section: Timer/Counter

let mut timer = Timer::new(board.TIMER0);

board.display_pins.col1.set_low().unwrap();

let mut row1 = board.display_pins.row1;

loop {
    row1.set_low().unwrap();
    rprintln!("OFF");
    timer.delay_ms(1000u16);
    row1.set_high().unwrap();
    rprintln!("ON");
    timer.delay_ms(1000u16);
}


}
