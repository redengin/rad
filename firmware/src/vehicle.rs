
pub trait Vehicle {
    fn gps_serial_read_async(&mut self, buffer: &mut[u8]);
}