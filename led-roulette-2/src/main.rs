#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::board::Board;
use microbit::hal::prelude::*;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // the mut board is the entire board of microbit v2 - the BSP aka Board Support Package which abstracts away the entire board
    
    let mut board = Board::take().unwrap();
    
    board.display_pins.col1.set_low().unwrap();
    board.display_pins.row1.set_high().unwrap();

    loop {}
}
