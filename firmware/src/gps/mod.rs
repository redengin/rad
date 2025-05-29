//! Generic GPS support - GPS Drivers implement GpsReader traits.
//! 
//! Upon initialization, GPS Drivers should leave the GPS hardware
//!     in low-power mode (if possible)
//! 
use crate::{Float};

// #[derive(Debug, Clone, Copy, Default)]
#[derive(Default)]
pub struct GpsData {
    pub latitude: Float,
    pub longitude: Float,
    pub altitude: Float,
    pub satellite_count: usize,
}

pub trait Gps {
    /// put the GPS driver into sensing mode
    fn start(&self);

    /// retrieve the GPS data
    fn get_data(&self) -> Option<GpsData>;

    /// put the GPS hardware into low power mode
    fn stop(&self);
}

// provide a dummy implementation
pub mod gps_dummy;