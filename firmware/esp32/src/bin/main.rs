#![no_std]
#![no_main]
use esp_backtrace as _; // implements panic
// override panic's halt to perform a software reset
#[unsafe(no_mangle)]
pub extern "C" fn custom_halt() { esp_hal::reset::software_reset(); }

use esp_alloc as _;

use rad_drone::time::{Duration, Timer};


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
    esp_println::logger::init_logger_from_env();

    spawner.spawn(ping()).unwrap();
    spawner.spawn(pong()).unwrap();
}

#[embassy_executor::task]
async fn ping()
{
    loop
    {
        esp_println::println!("PING");
        Timer::after(Duration::from_millis(1_000)).await;
    }
}

#[embassy_executor::task]
async fn pong()
{
    loop
    {
        esp_println::println!("PONG");
        Timer::after(Duration::from_millis(1_000)).await;
        panic!();
    }
}
