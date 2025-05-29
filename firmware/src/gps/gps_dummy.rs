//! Example Implementation of a GPS driver
//! 
//! Provides no GPS data

use crate::log;
use crate::gps;

pub struct GpsDummy {
    // Real implementation would take hardware resources
}


impl gps::Gps for GpsDummy {

    fn start(&self) { }

    fn get_data(&self) -> Option<gps::GpsData> {
        log::debug!("Dummy GPS data");
        Some(gps::GpsData::default())
    }

    fn stop(&self) { }
}