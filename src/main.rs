#![no_std]
#![no_main]

mod util;

#[arduino_hal::entry]
fn main() -> ! {
    //setup
    util::lcd::setup();

    //loop
    loop {
        util::lcd::_loop()
    }
}
