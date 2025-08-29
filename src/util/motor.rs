
use arduino_hal::port::mode::*;
use arduino_hal::port::*;
use arduino_hal::simple_pwm::{Timer2Pwm};


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
        mut in1 : Pin<Output, D22>,
        mut in2 : Pin<Output, D23>,
        mut in3 : Pin<Output, D24>,
        mut in4 : Pin<Output, D25>,
        mut pwm : Pin<PwmOutput<Timer2Pwm>, D9>,
    ) -> Self {
        in1.set_low();
        in2.set_low();
        in3.set_low();
        in4.set_low();
        pwm.enable();
        pwm.set_duty(128);
        Self {
            in1,
            in2,
            pwm,
            in3,
            in4,
            speed : 128,
            motor_status: MotorStatus::Forward,
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
        match self.motor_status { //TODO
            MotorStatus::Forward => {
                self.in1.set_high();
                self.in2.set_low();
                self.in3.set_low();
                self.in4.set_high();
            },
            MotorStatus::Backward => {
                self.in1.set_low();
                self.in2.set_high();
                self.in3.set_high();
                self.in4.set_low();
            },
            MotorStatus::TurnRight => {
                self.in1.set_high();
                self.in2.set_low();
                self.in3.set_high();
                self.in4.set_low();
            }
            MotorStatus::TurnLeft => {
                self.in1.set_low();
                self.in2.set_high();
                self.in3.set_low();
                self.in4.set_high();
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

pub fn _loop(motor: &mut Motor){
}
