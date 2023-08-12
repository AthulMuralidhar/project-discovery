#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

mod calibration;
use crate::calibration::calc_calibration;
use crate::calibration::calibrated_measurement;

mod led;
use crate::led::Direction;
use crate::led::direction_to_led;

use microbit::{display::blocking::Display, hal::Timer};

use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A};

use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();
    let i2c = { twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100) };
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz10).unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();
    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    let calibration = calc_calibration(&mut sensor, &mut display, &mut timer);
    rprintln!("Calibration: {:?}", calibration);
    rprintln!("Calibration done, entering busy loop");

    loop {
        while !sensor.mag_status().unwrap().xyz_new_data {}
        let mut data = sensor.mag_data().unwrap();
        data = calibrated_measurement(data, &calibration);

        let direction = match (data.x > 0, data.y > 0) {
                        // Quadrant I
                        (true, true) => Direction::NorthEast,
                        // Quadrant II
                        (false, true) => Direction::NorthWest,
                        // Quadrant III
                        (false, false) => Direction::SouthWest,
                        // Quadrant IV
                        (true, false) => Direction::SouthEast,
        };
                // use the led module to turn the direction into an LED arrow
        // and the led display functions from chapter 5 to display the
        // arrow
        display.show(&mut timer, direction_to_led(direction), 100);
    }

}