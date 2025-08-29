use core::fmt::Write;
use i2c_character_display::{CharacterDisplayPCF8574T, LcdDisplayType};
use arduino_hal::{I2c, Delay, delay_ms, Peripherals};

static mut LCD: Option<CharacterDisplayPCF8574T<I2c, Delay>> = None;


pub fn setup(i2c: I2c) {
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

#[deprecated(note = "이 함수는 딜레이를 포함하고 있어 성능에 영향을 줄 수 있습니다.")]
pub fn _loop(){
    unsafe {
        let mut lcd = LCD.as_mut().unwrap();
        delay_ms(2000); 
        lcd.set_cursor(0, 0);
        lcd.print("       WALL-");
        lcd.write_char('\u{0}');
    }
}
