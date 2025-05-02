#![no_std]

// re-export shared dependencies
pub use log;
pub use embassy_time as time;

use time::{Duration, Timer};

pub fn start(spawner: embassy_executor::Spawner)
{
    // initialize logger
    log::info!("dummy starting....");

    spawner.spawn(dummy()).unwrap();
}

#[embassy_executor::task]
async fn dummy()
{
    loop
    {
        log::debug!("dummy working....");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}