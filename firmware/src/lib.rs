#![no_std]

// re-export shared dependencies
pub use log;
pub use embassy_time as time;

// export rad_drone abstractions
// pub use gps;
// pub use imu;
// pub use motors;


pub fn start(spawner: embassy_executor::Spawner)
{
    // initialize logger
    log::info!("dummy starting....");

    spawner.spawn(dummy()).unwrap();
}

use time::{Duration, Timer};
#[embassy_executor::task]
async fn dummy()
{
    loop
    {
        log::debug!("dummy working....");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}