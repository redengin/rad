//! GPS
///=============================================================================

/// GPS drivers *should* (if possible) leave the hardware in low-power mode
pub trait GpsDriver {
    /// called before use to allow the driver to disable low-power mode
    fn enable(&self);

    /// put the driver into low-power mode
    fn disable(&self);
}

pub struct Gps {
    /// GPS data parsed from the NMEA sentences
    nmea: nmea::Nmea,
    /// serial interface
    // serial: Box<dyn GpsDriver>,
    serial: bool
}

impl Gps {

}