use error::*;
use sysfs_pwm::Pwm;

const PERIOD: u32 = 20_000_000;
const MAX_DUTY_CYCLE: u32 = 2_000_000;
const MIN_DUTY_CYCLE: u32 = 1_000_000;

pub struct Rover {
    left: Pwm,
    right: Pwm,
}

impl Rover {
    /// Creates a new rovers with both motors enabled but stopped. The motors
    /// will be disabled and the underlying pwm drivers unexported when the
    /// rover is dropped.
    pub fn new(chip: u32, left_pin: u32, right_pin: u32) -> Result<Rover> {
        let left = Pwm::new(chip, left_pin).chain_err(|| "failed to create left motor")?;
        let right = Pwm::new(chip, right_pin).chain_err(|| "failed to create right motor")?;
        Ok(Rover {
            left: left,
            right: right,
        })
    }

    /// Exports and setup the period for the servos.
    pub fn export(&self) -> Result<()> {
        self.left.export().chain_err(|| "failed to export the left motor pwm channel")?;
        self.right.export().chain_err(|| "failed to export the right motor pwm channel")?;
        self.left.set_period_ns(PERIOD).chain_err(|| "failed to set period on left motor")?;
        self.right.set_period_ns(PERIOD).chain_err(|| "failed to set period on right motor")
    }

    /// Enables/disables the motor. When disabled they keep their current
    /// speed and their speed can still be set but they will not move until
    /// enabled.
    pub fn enable(&self, enabled: bool) -> Result<()> {
        self.left.enable(enabled).chain_err(|| "failed to enable left motor")?;
        self.right.enable(enabled).chain_err(|| "failed to enable right motor")
    }

    /// Converts a speed between -100 (full reverse) and 100 (full forward)
    /// to a duty cycle.
    fn speed_to_duty_cycle(speed: i8) -> u32 {
        let duty_cycle = (((speed as i32 * 10000) + MIN_DUTY_CYCLE as i32) as u32 / 2) +
                         MIN_DUTY_CYCLE;
        if duty_cycle > MAX_DUTY_CYCLE {
            return MAX_DUTY_CYCLE;
        }
        if duty_cycle < MIN_DUTY_CYCLE {
            return MIN_DUTY_CYCLE;
        }
        duty_cycle
    }

    /// Sets the speed of the left motor. Can be any value between -100 (full
    /// reverse) and 100 (full forward), values above or below these limits will
    /// be to to the limit.
    pub fn set_left_speed(&self, speed: i8) -> Result<()> {
        self.left
            .set_duty_cycle_ns(Rover::speed_to_duty_cycle(-speed))
            .chain_err(|| "failed to set duty on left motor")
    }

    /// Sets the speed of the right motor. Can be any value between -100 (full
    /// reverse) and 100 (full forward), values above or below these limits will
    /// be to to the limit.
    pub fn set_right_speed(&self, speed: i8) -> Result<()> {
        self.right
            .set_duty_cycle_ns(Rover::speed_to_duty_cycle(speed))
            .chain_err(|| "failed to set duty on left motor")
    }

    /// Stops both the motors, equlivent to setting their speeds to 0.
    pub fn stop(&self) -> Result<()> {
        self.set_left_speed(0)?;
        self.set_right_speed(0)
    }

    /// Sets the speed of left and right motor. Can be any value between -100 (full
    /// reverse) and 100 (full forward), values above or below these limits will
    /// be to to the limit.
    pub fn set_speed(&self, left: i8, right: i8) -> Result<()> {
        self.set_left_speed(left)?;
        self.set_right_speed(right)
    }

    /// Unexports the motors so they can no longer be used
    pub fn unexport(&self) -> Result<()> {
        self.left.unexport().chain_err(|| "failed to unexport left motor")?;
        self.right.unexport().chain_err(|| "failed to unexport right motor")
    }
}