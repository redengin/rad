#![no_std]
//! Hardware Configuration for the Vehicle
use esp_hal::gpio::interconnect::{PeripheralInput, PeripheralOutput};
use esp_hal::peripheral::Peripheral;
use esp_hal::Async;
use esp_hal::{dma, dma_buffers};
///=============================================================================
use esp_hal::{mcpwm, spi, uart};

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

pub fn imu_spi_config() -> spi::master::Config {
    // TODO use a rad_drone imu driver's config values
    spi::master::Config::default()
        .with_frequency(esp_hal::time::RateExtU32::kHz(100))
        .with_mode(spi::Mode::_0)
}

// TODO use rad_drone esc driver's config value
// const ESC_PWM_HZ: u32 = 50;
// TODO use rad_drone esc driver's config value
// const ESC_MAX_DUTY_CYCLE: u16 = 100;
/// determines rate of the pwm master clock
///     (this * ESC_PWM_HZ) = master clock frequency
///     (lower pwm master clock frequency is expected to conserve power)
// const ESC_PWM_MASTER_OVERSAMPLING_SCALAR: u32 = 1; // Nyquist = 2

/// rad_drone::Vehicle
pub struct Esp32Drone {
    // gps: uart::Uart<'static, Async>,
    // imu: spi::master::SpiDmaBus<'static, Async>,
    // FIXME what is a PwmPin
    // esc_a_pwm: mcpwm::operator::PwmPin<'static, dyn mcpwm::PwmPeripheral, 0, true>,
}

impl Esp32Drone {
    pub fn new<SpiDmaCh>(
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
        // pwm_a: impl Peripheral<P = PWMA> + 'static,
        _pwm_a: impl Peripheral<P = impl mcpwm::PwmPeripheral> + 'static,
        _pwm_b: impl Peripheral<P = impl mcpwm::PwmPeripheral> + 'static,
        _esc_a_pin: impl Peripheral<P = impl PeripheralOutput> + 'static,
        _esc_b_pin: impl Peripheral<P = impl PeripheralOutput> + 'static,
        _esc_c_pin: impl Peripheral<P = impl PeripheralOutput> + 'static,
        _esc_d_pin: impl Peripheral<P = impl PeripheralOutput> + 'static,
    ) -> Self
    where
        SpiDmaCh: dma::DmaChannelFor<spi::AnySpi>,
    {
        // configure GPS hardware interface
        let gps_uart = uart::Uart::new(gps_uart, gps_uart_config())
            .unwrap()
            .with_tx(gps_tx_pin)
            .with_rx(gps_rx_pin)
            .into_async();

        // configure IMU hardware interface
        // create DMA buffers
        let (rx_buffer, rx_descriptors, tx_buffer, tx_descriptors) = dma_buffers!(32000);
        let dma_rx_buf = dma::DmaRxBuf::new(rx_descriptors, rx_buffer).unwrap();
        let dma_tx_buf = dma::DmaTxBuf::new(tx_descriptors, tx_buffer).unwrap();
        let imu = spi::master::Spi::new(imu_spi, imu_spi_config())
            .unwrap()
            .with_sck(imu_sck) // IMU SPI-CLK
            .with_mosi(imu_mosi) // IMU SPI-MOSI
            .with_miso(imu_miso) // IMU SPI-MISO
            // .with_cs(peripherals.GPIO5)  // IMU SPI-CS
            .with_dma(imu_dma)
            .with_buffers(dma_rx_buf, dma_tx_buf)
            .into_async();

        // configure ESC hardware interfaces
        // let pwm_clock_cfg = mcpwm::PeripheralClockConfig::with_frequency(
        //     esp_hal::time::RateExtU32::Hz(ESC_PWM_HZ * ESC_PWM_MASTER_OVERSAMPLING_SCALAR),
        // )
        // .unwrap();
        // let timer_clock_cfg = pwm_clock_cfg
        //     .timer_clock_with_frequency(
        //         ESC_MAX_DUTY_CYCLE, // output duty cycle range [0..ESC_MAX_DUTY_CYCLE]
        //         mcpwm::timer::PwmWorkingMode::Increase,
        //         esp_hal::time::RateExtU32::Hz(ESC_PWM_HZ),
        //     )
        //     .unwrap();
        // // create 3 pwm controllers on first mcpwm device
        // let mut mcpwm_a = mcpwm::McPwm::new(pwm_a, pwm_clock_cfg);
        // // configure timer0 (default for all operator channels)
        // mcpwm_a.timer0.start(timer_clock_cfg);
        // let esc_a_pwm = mcpwm_a.operator0.with_pin_a(
        //     esc_a_pin, // MOTOR-A speed controller pin
        //     mcpwm::operator::PwmPinConfig::UP_ACTIVE_HIGH,
        // );
        // let mut esc_b_pwm = mcpwm_a.operator1.with_pin_a(
        //     esc_b_pin, // MOTOR-B speed controller pin
        //     mcpwm::operator::PwmPinConfig::UP_ACTIVE_HIGH,
        // );
        // let mut esc_c_pwm = mcpwm_a.operator2.with_pin_a(
        //     esc_c_pin, // MOTOR-B speed controller pin
        //     mcpwm::operator::PwmPinConfig::UP_ACTIVE_HIGH,
        // );
        // // create 1 pwm controllers on second mcpwm device
        // let mut mcpwm_b = mcpwm::McPwm::new(pwm_b, pwm_clock_cfg);
        // // configure timer0 (default for all operator channels)
        // mcpwm_b.timer0.start(timer_clock_cfg);
        // let mut esc_d_pwm = mcpwm_b.operator0.with_pin_a(
        //     esc_d_pin, // MOTOR-A speed controller pin
        //     mcpwm::operator::PwmPinConfig::UP_ACTIVE_HIGH,
        // );

        Esp32Drone {
            // gps: gps_uart,
            // imu: imu,
            // FIXME
            // esc_a_pwm: esc_a_pwm,
        }
    }
}

/// provide the Vehicle abstraction
impl rad_drone::vehicle::Vehicle for Esp32Drone {
    fn gps_read(&mut self, _buf: &mut[u8]) {
        // FIXME
        // self.gps.read_async(buf)
    }

    fn imu_read_gyroscopes(&self) -> rad_drone::Vector3 {
        // TODO
        rad_drone::Vector3{x:0.0, y:0.0, z:0.0}
    }

    fn imu_read_accelerometers(&self) -> rad_drone::Vector3 {
        // TODO
        rad_drone::Vector3{x:0.0, y:0.0, z:0.0}
    }

    fn imu_read_magnetometers(&self) -> rad_drone::Vector3 {
        // TODO
        rad_drone::Vector3{x:0.0, y:0.0, z:0.0}
    }

    fn motor_a_duty(&self, _duty_cycle: u8) {
       // TODO 
    }

    fn motor_b_duty(&self, _duty_cycle: u8) {
       // TODO 
    }

    fn motor_c_duty(&self, _duty_cycle: u8) {
       // TODO 
    }

    fn motor_d_duty(&self, _duty_cycle: u8) {
       // TODO 
    }
}
