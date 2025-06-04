#![no_std]
//! Hardware Configuration for the Vehicle
///=============================================================================
use esp_hal::dma::DmaChannelFor;
use esp_hal::gpio::interconnect::{PeripheralInput, PeripheralOutput};
use esp_hal::mcpwm;
use esp_hal::peripheral::Peripheral;
use esp_hal::spi;
use esp_hal::uart;

// TODO use a rad_drone gps driver's data
pub fn gps_uart_config() -> uart::Config {
    // TODO use radrone GPS driver config values
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
    // TODO use a rad_drone imu driver's config values
    esp_hal::spi::master::Config::default()
        .with_frequency(esp_hal::time::RateExtU32::kHz(100))
        .with_mode(esp_hal::spi::Mode::_0)
}

// TODO use rad_drone esc driver's config value
const ESC_PWM_HZ: u32 = 50;
// TODO use rad_drone esc driver's config value
const ESC_MAX_DUTY_CYCLE: u16 = 100;
/// determines rate of the pwm master clock
///     (this * ESC_PWM_HZ) = master clock frequency
///     (lower pwm master clock frequency is expected to conserve power)
const ESC_PWM_MASTER_OVERSAMPLING_SCALAR: u32 = 1; // Nyquist = 2

/// rad_drone::Vehicle
pub struct Esp32Drone {
    gps: uart::Uart<'static, esp_hal::Async>,
    imu: spi::master::SpiDmaBus<'static, esp_hal::Async>,
    // FIXME what is a PwmPin
    // esc_a_pwm: dyn mcpwm::PwmPeripheral,
}

impl Esp32Drone {
    pub fn new<SpiDmaCh, PWMA, PWMB>(
        // gps
        gps_uart: impl Peripheral<P = impl uart::Instance> + 'static,
        gps_tx_pin: impl Peripheral<P = impl PeripheralOutput> + 'static,
        gps_rx_pin: impl Peripheral<P = impl PeripheralInput> + 'static,
        // imu
        imu_spi: impl Peripheral<P = impl spi::master::PeripheralInstance> + 'static,
        imu_dma: impl Peripheral<P = SpiDmaCh> + 'static,
        imu_sck: impl Peripheral<P = impl PeripheralOutput> + 'static,
        imu_mosi: impl Peripheral<P = impl PeripheralOutput> + 'static,
        imu_miso: impl Peripheral<P = impl PeripheralInput> + 'static,
        // motors
        pwm_a: impl Peripheral<P = PWMA> + 'static,
        pwm_b: impl Peripheral<P = PWMB> + 'static,
        esc_a_pin: impl Peripheral<P = impl PeripheralOutput> + 'static,
        esc_b_pin: impl Peripheral<P = impl PeripheralOutput> + 'static,
        esc_c_pin: impl Peripheral<P = impl PeripheralOutput> + 'static,
        esc_d_pin: impl Peripheral<P = impl PeripheralOutput> + 'static,
    ) -> Self
    where
        SpiDmaCh: DmaChannelFor<spi::AnySpi>,
        PWMA: mcpwm::PwmPeripheral,
        PWMB: mcpwm::PwmPeripheral,
    {
        // configure GPS hardware interface
        let gps_uart = uart::Uart::new(gps_uart, gps_uart_config())
            .unwrap()
            .with_tx(gps_tx_pin)
            .with_rx(gps_rx_pin)
            .into_async();

        // configure IMU hardware interface
        // create DMA buffers
        let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = esp_hal::dma_buffers!(32000);
        let dma_rx_buf = esp_hal::dma::DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
        let dma_tx_buf = esp_hal::dma::DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();
        let imu = esp_hal::spi::master::Spi::new(imu_spi, imu_spi_config())
            .unwrap()
            .with_sck(imu_sck) // IMU SPI-CLK
            .with_mosi(imu_mosi) // IMU SPI-MOSI
            .with_miso(imu_miso) // IMU SPI-MISO
            // .with_cs(peripherals.GPIO5)  // IMU SPI-CS
            .with_dma(imu_dma)
            .with_buffers(dma_rx_buf, dma_tx_buf)
            .into_async();

        // configure ESC hardware interfaces
        let pwm_clock_cfg = mcpwm::PeripheralClockConfig::with_frequency(
            esp_hal::time::RateExtU32::Hz(ESC_PWM_HZ * ESC_PWM_MASTER_OVERSAMPLING_SCALAR),
        )
        .unwrap();
        let timer_clock_cfg = pwm_clock_cfg
            .timer_clock_with_frequency(
                ESC_MAX_DUTY_CYCLE, // output duty cycle range [0..ESC_MAX_DUTY_CYCLE]
                mcpwm::timer::PwmWorkingMode::Increase,
                esp_hal::time::RateExtU32::Hz(ESC_PWM_HZ),
            )
            .unwrap();
        // create 3 pwm controllers on first mcpwm device
        let mut mcpwmA = mcpwm::McPwm::new(pwm_a, pwm_clock_cfg);
        // configure timer0 (default for all operator channels)
        mcpwmA.timer0.start(timer_clock_cfg);
        let mut esc_a_pwm = mcpwmA.operator0.with_pin_a(
            esc_a_pin, // MOTOR-A speed controller pin
            esp_hal::mcpwm::operator::PwmPinConfig::UP_ACTIVE_HIGH,
        );
        let mut esc_b_pwm = mcpwmA.operator1.with_pin_a(
            esc_b_pin, // MOTOR-B speed controller pin
            esp_hal::mcpwm::operator::PwmPinConfig::UP_ACTIVE_HIGH,
        );
        let mut esc_c_pwm = mcpwmA.operator2.with_pin_a(
            esc_c_pin, // MOTOR-B speed controller pin
            esp_hal::mcpwm::operator::PwmPinConfig::UP_ACTIVE_HIGH,
        );
        // create 1 pwm controllers on second mcpwm device
        let mut mcpwmB = mcpwm::McPwm::new(pwm_b, pwm_clock_cfg);
        // configure timer0 (default for all operator channels)
        mcpwmB.timer0.start(timer_clock_cfg);
        let mut esc_d_pwm = mcpwmB.operator0.with_pin_a(
            esc_d_pin, // MOTOR-A speed controller pin
            esp_hal::mcpwm::operator::PwmPinConfig::UP_ACTIVE_HIGH,
        );

        Esp32Drone {
            gps: gps_uart,
            imu: imu,
            // esc_a_pwm: esc_a_pwm,
        }
    }
}
