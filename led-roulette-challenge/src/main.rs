
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};


#[entry]
fn main() -> !{
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

// FIRST ROW
    let light1_1 = [
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let light1_2 = [
        [0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let light1_3 = [
        [0, 0, 1, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let light1_4 = [
        [0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let light1_5 = [
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

// LAST COLUMN DOWN MOVEMENT
    let light2_5 = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let light3_5 = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let light4_5 = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 0],
    ];
    let light5_5 = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1],
    ];


// BOTTOM ROW MOVE TO LEFT
    let light5_4 = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0],
    ];
    let light5_3 = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0],
    ];
    let light5_2 = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 1, 0, 0, 0],
    ];
    let light5_1 = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
    ];

// CLIMB UPTO 1,1

    let light4_1 = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let light3_1 = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let light2_1 = [
        [0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];


loop {
    display.show(&mut timer, light1_1, 10);
    rprintln!("ON 1X1");
    timer.delay_ms(1u32);
    display.show(&mut timer, light1_2, 10);
    rprintln!("ON 1X2");
    timer.delay_ms(1u32);
    display.show(&mut timer, light1_3, 10);
    rprintln!("ON 1X3");
    timer.delay_ms(1u32);
    display.show(&mut timer, light1_4, 10);
    rprintln!("ON 1X4");
    timer.delay_ms(1u32);
    display.show(&mut timer, light1_5, 10);
    rprintln!("ON 1X5");
    timer.delay_ms(1u32);

    display.show(&mut timer, light2_5, 10);
    rprintln!("ON 2X5");
    timer.delay_ms(1u32);
    display.show(&mut timer, light3_5, 10);
    rprintln!("ON 3X5");
    timer.delay_ms(1u32);
    display.show(&mut timer, light4_5, 10);
    rprintln!("ON 4X5");
    timer.delay_ms(1u32);
    display.show(&mut timer, light5_5, 10);
    rprintln!("ON 5X5");
    timer.delay_ms(1u32);


    display.show(&mut timer, light5_4, 10);
    rprintln!("ON 5X4");
    timer.delay_ms(1u32);
    display.show(&mut timer, light5_3, 10);
    rprintln!("ON 5X3");
    timer.delay_ms(1u32);
    display.show(&mut timer, light5_2, 10);
    rprintln!("ON 5X2");
    timer.delay_ms(1u32);
    display.show(&mut timer, light5_1, 10);
    rprintln!("ON 5X1");
    timer.delay_ms(1u32);

    display.show(&mut timer, light4_1, 10);
    rprintln!("ON 4X1");
    timer.delay_ms(1u32);
    display.show(&mut timer, light3_1, 10);
    rprintln!("ON 3X1");
    timer.delay_ms(1u32);
    display.show(&mut timer, light2_1, 10);
    rprintln!("ON 2X1");
    timer.delay_ms(1u32);

}


}
