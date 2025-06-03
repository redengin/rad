#![no_std]
#![no_main]
// use re-exported dependencies
use rad_drone::log;
// use rad_drone::time::{Duration, Timer};

// provide no_std support
use esp_backtrace as _; // implements panic
                        // override panic's halt to perform a software reset
#[unsafe(no_mangle)]
pub extern "C" fn custom_halt() {
    esp_hal::reset::software_reset();
}
// provide required alloc implementation
use esp_alloc as _;

use vehicle;

#[esp_hal_embassy::main]
async fn main(_spawner: embassy_executor::Spawner) {
    // initialize SoC
    let peripherals = esp_hal::init(
        esp_hal::Config::default(), // .with_cpu_clock(esp_hal::clock::CpuClock::max())
    );

    // initialize embassy scheduler
    #[cfg(feature = "esp32")]
    {
        let timg1 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG1);
        esp_hal_embassy::init(timg1.timer0);
    }
    #[cfg(not(feature = "esp32"))]
    {
        use esp_hal::timer::systimer::SystemTimer;
        let systimer = SystemTimer::new(peripherals.SYSTIMER);
        esp_hal_embassy::init(systimer.alarm0);
    }

    // initialize logger
    #[cfg(debug_assertions)]
    esp_println::logger::init_logger(log::LevelFilter::Debug);
    #[cfg(not(debug_assertions))]
    esp_println::logger::init_logger(log::LevelFilter::Info);

    // create the vehicle
    // connect the GPS
    let gps_uart = esp_hal::uart::Uart::new(peripherals.UART0, vehicle::gps_uart_config())
        .unwrap()
        .with_tx(peripherals.GPIO1) // GPS Tx Pin
        .with_rx(peripherals.GPIO3) // GPS Rx Pin
        .into_async();
    // connect the IMU
    let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = esp_hal::dma_buffers!(32000);
    let dma_rx_buf = esp_hal::dma::DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
    let dma_tx_buf = esp_hal::dma::DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();
    let mut imu_spi = esp_hal::spi::master::Spi::new(peripherals.SPI2, vehicle::imu_spi_config())
        .unwrap()
        .with_sck(peripherals.GPIO14) // IMU SPI-CLK
        .with_mosi(peripherals.GPIO13) // IMU SPI-MOSI
        .with_miso(peripherals.GPIO12) // IMU SPI-MISO
        // .with_cs(peripherals.GPIO5)  // IMU SPI-CS
        .with_dma(peripherals.DMA_SPI2)
        .with_buffers(dma_rx_buf, dma_tx_buf)
        .into_async();
    // connect the speed controllers
    let pwm_clock_cfg =
        esp_hal::mcpwm::PeripheralClockConfig::with_frequency(esp_hal::time::RateExtU32::MHz(32))
            .unwrap();
    let mut mcpwm0 = esp_hal::mcpwm::McPwm::new(peripherals.MCPWM0, pwm_clock_cfg);
    mcpwm0.operator0.set_timer(&mcpwm0.timer0);
    let mut escA_pwm = mcpwm0.operator0.with_pin_a(
        peripherals.GPIO8,  // MOTOR-A speed controller pin
        esp_hal::mcpwm::operator::PwmPinConfig::UP_ACTIVE_HIGH,
    );
    mcpwm0.operator1.set_timer(&mcpwm0.timer1);
    let mut escB_pwm = mcpwm0.operator1.with_pin_a(
        peripherals.GPIO9,  // MOTOR-B speed controller pin
        esp_hal::mcpwm::operator::PwmPinConfig::UP_ACTIVE_HIGH,
    );
    mcpwm0.operator2.set_timer(&mcpwm0.timer2);
    let mut escC_pwm = mcpwm0.operator2.with_pin_a(
        peripherals.GPIO10,  // MOTOR-B speed controller pin
        esp_hal::mcpwm::operator::PwmPinConfig::UP_ACTIVE_HIGH,
    );
    let mut mcpwm1 = esp_hal::mcpwm::McPwm::new(peripherals.MCPWM1, pwm_clock_cfg);
    mcpwm1.operator0.set_timer(&mcpwm1.timer0);
    let mut escD_pwm = mcpwm1.operator0.with_pin_a(
        peripherals.GPIO11,  // MOTOR-B speed controller pin
        esp_hal::mcpwm::operator::PwmPinConfig::UP_ACTIVE_HIGH,
    );

    // // initialize the vehicle
    // let vehicle = rad_drone::Vehicle{
    //     // FIXME choose imu implementation
    //     imu: imu_dummy::new()
    // };

    // // start the rad_drone tasks
    // rad_drone::start(spawner, vehicle);
}

// use esp_hal::{peripherals::Peripherals, uart::Config};
// struct Esp32Drone {
//     gps_uart: esp_hal::uart::Uart<'static, esp_hal::Async>,
// }

// impl rad_drone::gps::GpsDriver for Esp32Drone {
//     fn enable(&self) {

//     }

//     fn disable(&self) {

//     }
// }
