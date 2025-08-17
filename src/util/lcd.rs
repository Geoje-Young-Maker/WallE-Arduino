use core::fmt::Write;
use i2c_character_display::{BaseCharacterDisplay, CharacterDisplayPCF8574T, LcdDisplayType};
use arduino_hal::{I2c, Delay};

static mut LCD: Option<CharacterDisplayPCF8574T<I2c, Delay>> = None;


pub fn setup() {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let sda = pins.d20.into_pull_up_input();
    let scl = pins.d21.into_pull_up_input();


    let i2c = I2c::new(
        dp.TWI,
        sda,
        scl,
        100_000,
    );

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
        lcd.print("       WALL-");
        lcd.write_char('\u{0}');
    }
}