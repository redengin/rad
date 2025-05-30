#![no_std]

// re-export shared dependencies
pub use embassy_time as time;
pub use log;


// export rad_drone abstractions
pub mod imu;
pub mod gps;

pub struct Vehicle {
    // imu:  &'r mut dyn crate::imu::Imu,
    // gps:  &'r mut dyn crate::gps::Gps,
}
mod flight_controller;
pub fn start(spawner: embassy_executor::Spawner, vehicle: Vehicle) {
    // dummy demo
    log::info!("dummy starting....");
    spawner.spawn(dummy()).unwrap();

    // start the flight controller
    log::info!("starting flight controller...");
    spawner.spawn(flight_controller::thread(vehicle)).unwrap();
    log::info!("flight controller started");
}

use time::{Duration, Timer};
#[embassy_executor::task]
async fn dummy() {
    loop {
        log::debug!("dummy working....");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

// Common Data Types
// -----------------------------------------------------------------------------

// choose floating point size
#[cfg(feature="high_precision")]
type Float = f64;
#[cfg(not(feature="high_precision"))]
type Float = f32;

// Three-Dimensional(x,y,z) data
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Vector3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
impl Vector3 {
    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self { x, y, z }
    }
}
// TODO provide no_std to_string()
