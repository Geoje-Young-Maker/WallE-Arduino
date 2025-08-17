use core::fmt::Write;
use i2c_character_display::{BaseCharacterDisplay, CharacterDisplayPCF8574T, LcdDisplayType};
use arduino_hal::{I2c, Delay, delay_ms, Peripherals, DefaultClock};
use arduino_hal::hal::{Atmega, Pins};
use arduino_hal::pac::TWI;
use arduino_hal::port::{Pin, D20, D21};
use arduino_hal::port::mode::{Input, PullUp};

static mut LCD: Option<CharacterDisplayPCF8574T<I2c, Delay>> = None;


pub fn setup(i2c: I2c) {

    let dp = Peripherals::take().unwrap();


   

    let delay = Delay::new();



    let mut lcd = CharacterDisplayPCF8574T::new(i2c, LcdDisplayType::Lcd20x2, delay);
    let reversede: [u8; 8] = [
        0b11111,
        0b10001,
        0b10111,
        0b10001,
        0b10111,
        0b10111,
        0b10001,
        0b11111,
    ];

    lcd.create_char(0, reversede);

    lcd.init();
    lcd.backlight(true);
    lcd.print("       WALL-");
    lcd.write_char('\u{0}');

    unsafe {
        LCD = Some(lcd);
    }
}

#[inline]
pub fn _loop(){
    unsafe {
        let mut lcd = LCD.as_mut().unwrap();
        delay_ms(2000);
        lcd.set_cursor(0, 0);
        lcd.print("       WALL-");
        lcd.write_char('\u{0}');
    }
}