#![no_std]
#![no_main]
// use re-exported dependencies
use rad_drone::log;
use rad_drone::time::{Duration, Timer};

// provide no_std support
use esp_backtrace as _; // implements panic
// override panic's halt to perform a software reset
#[unsafe(no_mangle)]
pub extern "C" fn custom_halt() { esp_hal::reset::software_reset(); }
use esp_alloc as _;


#[esp_hal_embassy::main]
async fn main(spawner: embassy_executor::Spawner)
{
    // initialize peripherals
    let peripherals = esp_hal::init(
        esp_hal::Config::default()
            // .with_cpu_clock(esp_hal::clock::CpuClock::max())
    );

    // initialize embassy scheduler
    #[cfg(feature = "esp32")] {
        let timg1 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG1);
        esp_hal_embassy::init(timg1.timer0);
    }
    #[cfg(not(feature = "esp32"))] {
        use esp_hal::timer::systimer::SystemTimer;
        let systimer = SystemTimer::new(peripherals.SYSTIMER);
        esp_hal_embassy::init(systimer.alarm0);
    }

    // initialize logger
#[cfg(debug_assertions)]
    esp_println::logger::init_logger(log::LevelFilter::Debug);
#[cfg(not(debug_assertions))]
    esp_println::logger::init_logger(log::LevelFilter::Info);

    // // initialize the vehicle
    // let vehicle = rad_drone::Vehicle{
    //     // FIXME choose imu implementation
    //     imu: imu_dummy::new()
    // };

    // // start the rad_drone tasks
    // rad_drone::start(spawner, vehicle);
}