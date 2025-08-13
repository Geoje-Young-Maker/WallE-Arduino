use core::fmt::Write;
use i2c_character_display::{CharacterDisplayPCF8574T, LcdDisplayType};

pub fn lcdmain() {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let sda = pins.d20.into_pull_up_input(); // SDA (PD1)
    let scl = pins.d21.into_pull_up_input(); // SCL (PD0)


    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        sda,
        scl,
        100_000,
    );

    let delay = arduino_hal::Delay::new();

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
    lcd.write_char(0u8 as char);

}