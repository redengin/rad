#![no_std]

// re-export shared dependencies
pub use embassy_time as time;
pub use log;


// export rad_drone abstractions for start()
// pub use gps;
// pub use imu;
// pub use motors;


// pub struct Location
// {
//     longitude:  f32,
//     latitude:   f32,
//     altitude:   f32,
// }


pub mod imu;
pub struct Vehicle {

}
mod flight_controller;
pub fn start(spawner: embassy_executor::Spawner, vehicle: Vehicle) {
    // dummy demo
    log::info!("dummy starting....");
    spawner.spawn(dummy()).unwrap();

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

// Quaternion https://en.wikipedia.org/wiki/Quaternion
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Quaternion {
    pub w: Float,
    pub x: Float,
    pub y: Float,
    pub z: Float,
}
// TODO provide no_std to_string()
impl Quaternion {
    // TODO use optimized math cargo
    pub fn rotate(&self, vector: Vector3) -> Vector3 {
        // Rotate a vector by a quaternion using the formula:
        // v' = q * v * q^-1
        // Where q^-1 is the conjugate since we assume unit quaternions
        let qw = self.w;
        let qx = self.x;
        let qy = self.y;
        let qz = self.z;
        let vx = vector.x;
        let vy = vector.y;
        let vz = vector.z;

        // Calculate rotation using quaternion multiplication
        let x = (1.0 - 2.0 * qy * qy - 2.0 * qz * qz) * vx
            + (2.0 * qx * qy - 2.0 * qz * qw) * vy
            + (2.0 * qx * qz + 2.0 * qy * qw) * vz;
        let y = (2.0 * qx * qy + 2.0 * qz * qw) * vx
            + (1.0 - 2.0 * qx * qx - 2.0 * qz * qz) * vy
            + (2.0 * qy * qz - 2.0 * qx * qw) * vz;
        let z = (2.0 * qx * qz - 2.0 * qy * qw) * vx
            + (2.0 * qy * qz + 2.0 * qx * qw) * vy
            + (1.0 - 2.0 * qx * qx - 2.0 * qy * qy) * vz;

        Vector3 { x, y, z }
    }
}