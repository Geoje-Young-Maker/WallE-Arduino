#![no_std]
#![no_main]
#![allow(unused_must_use)]

mod util;

use arduino_hal::{delay_ms, I2c, Peripherals};
use panic_halt as _;
use arduino_hal::simple_pwm::{IntoPwmPin, Timer2Pwm};
use arduino_hal::simple_pwm::Prescaler::Prescale64;
use i2c_character_display::{CharacterDisplayPCF8574T, LcdDisplayType};
use crate::util::moter::{Motor, MotorStatus};

#[no_mangle]
pub extern "C" fn delayms(ms: u16) {
    delay_ms(ms.into());
}

extern "C" {
    fn setupcpp();
    fn loopcpp();
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let sda = pins.d20.into_pull_up_input();
    let scl = pins.d21.into_pull_up_input();
    let mut timer = Timer2Pwm::new(dp.TC2, Prescale64);

    let i2c = I2c::new(dp.TWI, sda, scl, 100_000);

    unsafe {
        setupcpp();
    }

    let mut motor = Motor::new(
        pins.d22.into_output(),
        pins.d23.into_output(),
        pins.d24.into_output(),
        pins.d25.into_output(),
        pins.d9.into_output().into_pwm(&mut timer)
    );

    motor.setstatus(MotorStatus::Backward);
    motor.setspeed(255);
    motor.movemoter();

    util::lcd::setup(i2c);


    //loop
    loop {
        unsafe {
            loopcpp();
        }

        util::lcd::_loop();
    }
}
