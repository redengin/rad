//! Generic IMU support - IMU Drivers implement ImuReader traits.
//!
//! Upon initialization, IMU Drivers should leave the IMU hardware
//!   in low-power mode (if possible).

// use crate::{Float, Vector3, Quaternion};

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
    // fn get_data(&self) -> Option<ImuData>;

    /// put the IMU into low-power mode
    ///
    /// IMU driver should log errors
    fn stop(&self);
}

// provide a dummy implementation
pub mod imu_dummy;