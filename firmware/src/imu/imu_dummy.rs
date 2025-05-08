//! Example Implementation of an IMU driver
//!
//! Provides no IMU data

use crate::log;
use crate::imu::{ImuData, ImuReader};

pub struct ImuDummy {
    // Real implementation would use this for hardware resources and state
}

impl ImuReader for ImuDummy {
    /// Runs calibration routines
    fn calibrate(&self) -> bool {
        // Real implementation would use the hardware calibration routines    
        log::debug!("Dummy IMU calibrated");
        true
    }

    /// put the IMU into sensing mode
    fn start(&self) {
        // Real implementation would put the hardware into sensing mode
        log::debug!("Dummy IMU started");
    }

    /// Returns the most recent sensor data.
    fn get_data(&self) -> Option<ImuData> {
        log::debug!("Dummy IMU getting data");
        Some(ImuData::default())
    }
    
    /// Stops the reading thread.
    fn stop(&self) {
        // Real implementation would put the hardware into low-power mode
        log::debug!("Dummy IMU stopped");
    }
}

impl Drop for ImuDummy {
    /// upon release of an IMU driver, put the hardware into low-power mode
    fn drop(&mut self) {
        self.stop();
    }
}