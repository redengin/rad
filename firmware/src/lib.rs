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


// pub mod imu;

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
