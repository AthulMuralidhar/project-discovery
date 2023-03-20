#![deny(unsafe_code)]
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

// NOTES:
// 
// cargo run with --target thumbv7em-none-eabihf
// to check if the build is an ARM binary:  cargo readobj --target thumbv7em-none-eabihf --bin led-roulette -- --file-headers
// 
// debugging with gdb: 
// - it runs at localhost:1337 so, type: target remote :1337 in the gdb repl
// - should use cmd: gdb-multiarch with ubuntu - gdb with fedora. ex:  gdb-multiarch target/thumbv7em-none-eabihf/debug/led-roulette

#[entry]
fn main() -> ! {
    // notice the return type of the main fn
    // it is not a void but a "!" meaning, it can't return and terminate

    let  x = 42;
    let _y;
    _y = x;



// will try to halt gdb here lol
    loop {}
    
}
