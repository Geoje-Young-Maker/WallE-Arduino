#![no_std]
#![no_main]

mod util;

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    //setup
    util::lcd::setup();

    //loop
    loop {
        util::lcd::_loop()
    }
}
