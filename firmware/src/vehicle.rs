
pub trait Vehicle {
    fn gps_read_async(&mut self, buffer: &mut[u8]);
}