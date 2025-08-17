use arduino_hal::{Peripherals, Pins};
use arduino_hal::port::mode::*;
use arduino_hal::port::*;
use arduino_hal::simple_pwm::Prescaler::Prescale64;
use arduino_hal::simple_pwm::{IntoPwmPin, Timer2Pwm};


//PinInit

pub struct Motor{
    in1: Pin<Output, D22>,
    in2: Pin<Output, D23>,
    pwm: Pin<PwmOutput<Timer2Pwm>, D9>,
    in3: Pin<Output, D24>,
    in4: Pin<Output, D25>,
    speed: u8,
    motor_status: MotorStatus,
}

impl Motor {
    pub fn new(
        mut in1: Pin<Output, D22>,
        mut in2: Pin<Output, D23>,
        mut pwm: Pin<PwmOutput<Timer2Pwm>, D9>,
        mut in3: Pin<Output, D24>,
        mut in4: Pin<Output, D25>,
    ) -> Self {
        in1.set_low();
        in2.set_low();
        in3.set_low();
        in4.set_low();
        pwm.enable();
        Self {
            in1,
            in2,
            pwm,
            in3,
            in4,
            speed : 128,
            motor_status,
        }
    }
    pub fn setspeed(&mut self, n_speed: u8) -> &mut Self{
        self.speed = n_speed;
        self.pwm.set_duty(n_speed);
        self
    }
    pub fn setstatus(&mut self, motorstatus:MotorStatus) -> &mut Self{
        self.motor_status = motorstatus;
        self
    }
    pub fn movemoter(&mut self) {
        match self.motor_status {
            MotorStatus::Forward => {
                self.in1.set_high();
                self.in2.set_low();

            }
        }
    }
}

pub enum MotorStatus {
    Forward,
    Backward,
    TurnLeft,
    TurnRight,
}


static mut MOTOR: Option<Motor> = None;

pub fn setup(){

    let dp:Peripherals = Peripherals::take().unwrap();
    let pins: Pins = arduino_hal::pins!(dp);
    let mut timer: Timer2Pwm = Timer2Pwm::new(dp.TC2, Prescale64);

}

pub fn _loop(){

}