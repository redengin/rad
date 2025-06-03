#![no_std]
//! Hardware Configuration for the Vehicle
///=============================================================================
use esp_hal::uart;

// TODO use a rad_drone gps driver's data
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

// TODO use a rad_drone imu driver's data
pub fn imu_spi_config() -> esp_hal::spi::master::Config {
    esp_hal::spi::master::Config::default()
        .with_frequency(esp_hal::time::RateExtU32::kHz(100))
        .with_mode(esp_hal::spi::Mode::_0)
}

// pub fn esc_pwm_freq -> esp_hal::time::Rate {
//     esp_hal::time::RateExtU32::Hz(50)
// }

pub struct Esp32Drone {
    gps: esp_hal::uart::Uart<'static, esp_hal::Async>,
    imu: esp_hal::spi::master::Spi<'static, esp_hal::Async>,
}

use esp_hal::gpio::interconnect::{PeripheralInput, PeripheralOutput};
use esp_hal::peripheral::Peripheral;
// use esp_hal::uart;
use esp_hal::spi;
use esp_hal::mcpwm;

impl Esp32Drone {
    pub fn new(
        // gps
        gps_uart: impl Peripheral<P = impl uart::Instance> + 'static,
        gps_tx_pin: impl Peripheral<P = impl PeripheralOutput> + 'static,
        gps_rx_pin: impl Peripheral<P = impl PeripheralInput> + 'static,
        // imu
        imu_spi: impl Peripheral<P = impl spi::master::PeripheralInstance> + 'static,
        // imu_dma: impl Peripheral<P = spi::master::SpiDma + 'static,
        imu_sck: impl Peripheral<P = impl PeripheralOutput> + 'static,
        imu_mosi: impl Peripheral<P = impl PeripheralOutput> + 'static,
        imu_miso: impl Peripheral<P = impl PeripheralInput> + 'static,
        // motors
        // FIXME mcpwm (both 0 and 1)
        _esc_a_pin: impl Peripheral<P = impl PeripheralOutput> + 'static,
        _esc_b_pin: impl Peripheral<P = impl PeripheralOutput> + 'static,
        _esc_c_pin: impl Peripheral<P = impl PeripheralOutput> + 'static,
        _esc_d_pin: impl Peripheral<P = impl PeripheralOutput> + 'static,
    ) -> Self {
        // configure GPS hardware interface
        let gps_uart = uart::Uart::new(gps_uart, gps_uart_config())
            .unwrap()
            .with_tx(gps_tx_pin)
            .with_rx(gps_rx_pin)
            .into_async();
        // configure IMU hardware interface
        // create DMA buffers
        // let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = esp_hal::dma_buffers!(32000);
        // let dma_rx_buf = esp_hal::dma::DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
        // let dma_tx_buf = esp_hal::dma::DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();
        let imu = esp_hal::spi::master::Spi::new(imu_spi, imu_spi_config())
            .unwrap()
            .with_sck(imu_sck) // IMU SPI-CLK
            .with_mosi(imu_mosi) // IMU SPI-MOSI
            .with_miso(imu_miso) // IMU SPI-MISO
            // .with_cs(peripherals.GPIO5)  // IMU SPI-CS
            // .with_dma(imu_dma)
            // .with_buffers(dma_rx_buf, dma_tx_buf)
            .into_async();

        Esp32Drone {
            gps: gps_uart,
            imu: imu,
        }
    }
}
