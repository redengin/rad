//! Generic IMU support - IMU Drivers implement ImuReader traits.
//!
//! Upon initialization, IMU Drivers should leave the IMU hardware
//!   in low-power mode (if possible).

use crate::{Float, Vector3, Quaternion};

/// standard IMU hardware data
/// 
/// IMU Drivers should only provide direct measurements
// #[derive(Debug, Clone, Copy, Default)]
#[derive(Default)]
pub struct ImuData {
    /// Acceleration including gravity (m/s²)
    pub accelerometer: Option<Vector3>,
    /// Angular velocity (deg/s)
    pub gyroscope: Option<Vector3>,
    /// Magnetic field vector (micro Tesla, µT)
    pub magnetometer: Option<Vector3>,
    /// Temperature (°C)
    pub temperature: Option<Float>,
// TODO provide an Orientation enum to handle variants
//------------------------------------------------------------------------------
    /// Orientation as a unit quaternion
    pub quaternion: Option<Quaternion>,
    /// Orientation as Euler angles (deg)
    pub euler: Option<Vector3>,
//------------------------------------------------------------------------------
    /// Linear acceleration (acceleration without gravity) (m/s²)
    pub linear_acceleration: Option<Vector3>,
    /// Estimated gravity vector (m/s²)
    pub gravity: Option<Vector3>,
}

pub trait Imu {
    /// Calibrate the IMU (only called while the vehicle is stationary)
    /// 
    /// IMU driver should log errors
    /// - run calibration routines
    /// - enter low-power mode
    fn calibrate(&self) -> bool;

    /// put the IMU into sensing mode
    ///
    /// IMU driver should log errors
    ///
    /// Will be called after initializing the IMU driver
    fn start(&self);

    /// Retrieves the latest available IMU data.
    /// 
    /// IMU driver should log errors
    fn get_data(&self) -> Option<ImuData>;

    /// put the IMU into low-power mode
    ///
    /// IMU driver should log errors
    fn stop(&self);
}

// provide a dummy implementation
pub mod imu_dummy;