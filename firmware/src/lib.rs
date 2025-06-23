#![no_std]

// re-export shared dependencies
pub use embassy_net as embassy_net;
pub use embassy_time as time;
pub use log;
pub mod vehicle;

use crate::vehicle::Vehicle;
pub fn start<VEHICLE>(_spawner: embassy_executor::Spawner, _vehicle: VEHICLE)
    where VEHICLE: Vehicle,
{

}

// mod flight_controller;
// pub fn start(spawner: embassy_executor::Spawner, vehicle: Vehicle) {
//     // dummy demo
//     log::info!("dummy starting....");
//     spawner.spawn(dummy()).unwrap();

//     // start the flight controller
//     // log::info!("starting flight controller...");
//     // spawner.spawn(flight_controller::thread(vehicle)).unwrap();
//     // log::info!("flight controller started");
// }

// use time::{Duration, Timer};
// #[embassy_executor::task]
// async fn dummy() {
//     loop {
//         log::debug!("dummy working....");
//         Timer::after(Duration::from_millis(1_000)).await;
//     }
// }

// Common Data Types
// -----------------------------------------------------------------------------

// choose floating point size
#[cfg(feature="high_precision")]
pub type Float = f64;
#[cfg(not(feature="high_precision"))]
pub type Float = f32;

pub struct Vector3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

// FIXME
// struct Position {
//     latitude:   Float,
//     longitude:  Float,
//     altitude:   Float,
// }
