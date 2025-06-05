
// use async_trait::async_trait;

use crate::Vector3;

pub trait Vehicle {
    /// read serial data into buffer
    fn gps_read(&mut self, buffer: &mut[u8]);

    /// read the gyroscopes state
    fn imu_read_gyroscopes(&self) -> Vector3;

    /// read the accelerometers state
    fn imu_read_accelerometers(&self) -> Vector3;

    /// read the magnetometers state
    fn imu_read_magnetometers(&self) -> Vector3;

    fn motor_a_duty(&self, duty_cycle: u8);
    fn motor_b_duty(&self, duty_cycle: u8);
    fn motor_c_duty(&self, duty_cycle: u8);
    fn motor_d_duty(&self, duty_cycle: u8);
}