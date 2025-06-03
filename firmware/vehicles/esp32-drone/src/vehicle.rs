#![no_std]
//! Hardware Configuration for the Vehicle
///=============================================================================
use esp_hal::uart;

pub fn gps_uart_config() -> uart::Config {
    uart::Config::default()
    // the following show the default configuration and the method to modify
    // .with_baudrate(115_200)
    // .with_data_bits(uart::DataBits::_8)
    // .with_parity(uart::Parity::None)
    // .with_stop_bits(uart::StopBits::_1)
    // .with_rx_fifo_full_threshold(120)
    // .with_rx_timeout(10)
}

pub fn imu_spi_config() -> esp_hal::spi::master::Config {
    esp_hal::spi::master::Config::default()
        .with_frequency(esp_hal::time::RateExtU32::kHz(100))
        .with_mode(esp_hal::spi::Mode::_0)
}

// pub fn esc_pwm_freq -> esp_hal::time::Rate {
//     esp_hal::time::RateExtU32::Hz(50)
// }