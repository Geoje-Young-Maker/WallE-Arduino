#![no_std]
#![no_main]

mod util;

use panic_halt as _;


#[arduino_hal::entry]
fn main() -> ! {
    util::lcd::lcdmain();
    loop {
    }
}
