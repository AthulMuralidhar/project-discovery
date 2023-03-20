#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
// use microbit as _;

// cargo run with --target thumbv7em-none-eabihf


#[entry]
fn main() -> ! {
    // notice the return type of the main fn
    // it is not a void but a "!" meaning, it can't return and terminate

    let  x = 42;
    let _y;
    _y = x;

    loop {}
    
}
