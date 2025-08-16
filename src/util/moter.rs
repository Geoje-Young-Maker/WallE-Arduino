use arduino_hal::{Peripherals, Pins};
use arduino_hal::port::mode::*;
use arduino_hal::port::*;
use arduino_hal::simple_pwm::Prescaler::Prescale64;
use arduino_hal::simple_pwm::{IntoPwmPin, Timer2Pwm};


//PinInit

struct Motor<IN1, IN2, PWM> {
    IN1: Pin<Output, IN1>,
    IN2: Pin<Output, IN2>,
    PWM: Pin<PwmOutput<Timer2Pwm>, PWM>
}

enum MotorStatus {
    Forward(u8), 
    Backward(u8),
    Stop,
}
static mut MOTOR1: Option<Motor<D22, D23, D9>> = None;
static mut MOTOR2: Option<Motor<D24, D25, D10>> = None;

pub fn setup(){

    let dp:Peripherals = Peripherals::take().unwrap();
    let pins: Pins = arduino_hal::pins!(dp);
    let mut timer: Timer2Pwm = Timer2Pwm::new(dp.TC2, Prescale64);
}

pub fn _loop(){

}