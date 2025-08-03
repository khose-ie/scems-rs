pub mod adc;
pub mod can;
pub mod flash;
pub mod i2c;
pub mod io;
pub mod spi;
pub mod uart;
pub mod wd;

pub trait PeriphDevice<T>
{
    fn new(handle: *mut T) -> Self;
    fn handle_value(&self) -> *mut T;
}
