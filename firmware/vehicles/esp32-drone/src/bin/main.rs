#![no_std]
#![no_main]

use core::str::FromStr;

// provide no_std support
use esp_backtrace as _; // implements panic
                        // override panic's halt to perform a software reset
#[unsafe(no_mangle)]
pub extern "C" fn custom_halt() {
    esp_hal::reset::software_reset();
}
// provide esp_hal required alloc implementation
use esp_alloc as _;

// use re-exported dependencies
use rad_drone::log;
use rad_drone::embassy_net;

// use local vehicle implementation
use vehicle::Esp32Drone;


// When you are okay with using a nightly compiler it's better to use https://docs.rs/static_cell/2.1.0/static_cell/macro.make_static.html
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

#[esp_hal_embassy::main]
async fn main(spawner: embassy_executor::Spawner) {
    // initialize SoC (with max compute - aka cpu_clock frequency)
    let peripherals =
        esp_hal::init(esp_hal::Config::default().with_cpu_clock(esp_hal::clock::CpuClock::max()));

    // initialize embassy scheduler
    #[cfg(feature = "esp32")]
    {
        let timg1 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG1);
        esp_hal_embassy::init(timg1.timer0);
    }
    #[cfg(not(feature = "esp32"))]
    {
        let systimer = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER);
        esp_hal_embassy::init(systimer.alarm0);
    }

    // initialize logger
    #[cfg(debug_assertions)]
    esp_println::logger::init_logger(log::LevelFilter::Debug);
    #[cfg(not(debug_assertions))]
    esp_println::logger::init_logger(log::LevelFilter::Info);
    
    // initialize networking
    esp_alloc::heap_allocator!(72 * 1024);
    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    let mut rng = esp_hal::rng::Rng::new(peripherals.RNG);
    let esp_wifi_ctrl = &*mk_static!(
        esp_wifi::EspWifiController<'static>,
        esp_wifi::init(timg0.timer0, rng.clone(), peripherals.RADIO_CLK).unwrap()
    );
    let (wifi_ap, _wifi_sta, _wifi_controller) =
        esp_wifi::wifi::new_ap_sta(&esp_wifi_ctrl, peripherals.WIFI).unwrap();
    // Init network stack
    let seed = (rng.random() as u64) << 32 | rng.random() as u64;
    let (stack, runner) = embassy_net::new(
        wifi_ap,
        // config,
        embassy_net::Config::ipv4_static(embassy_net::StaticConfigV4{
            address: embassy_net::Ipv4Cidr::new(embassy_net::Ipv4Address::from_str("192.168.0.1").unwrap(), 24),
            gateway: None,
            dns_servers: Default::default(),
        }),
        mk_static!(embassy_net::StackResources<3>, embassy_net::StackResources::<3>::new()),
        seed,
    );


    // create the vehicle
    let vehicle = Esp32Drone::new(
        // GPS interface
        peripherals.UART0, // GPS UART
        peripherals.GPIO1, // GPS Tx Pin
        peripherals.GPIO3, // GPS Tx Pin
        // IMU interface
        peripherals.SPI2,
        peripherals.DMA_SPI2,
        peripherals.GPIO14, // IMU SPI CLK
        peripherals.GPIO13, // IMU SPI MOSI
        peripherals.GPIO12, // IMU SPI MISO
        // motors ESC interface
        // FIXME
        peripherals.MCPWM0,
        peripherals.MCPWM1,
        peripherals.GPIO8,  // Motor A ESC pin
        peripherals.GPIO9,  // Motor B ESC pin
        peripherals.GPIO10, // Motor C ESC pin
        peripherals.GPIO11, // Motor D ESC pin
    );

    // start the rad_drone tasks
    rad_drone::start(spawner, vehicle);
}
