use core::u32;

pub mod adc;
pub mod can;
pub mod flash;
pub mod i2c;
pub mod io;
pub mod spi;
pub mod uart;
pub mod wd;

pub trait Handle<T>
where
    Self: Sized,
{
    fn handle_value(&self) -> *mut T;

    fn channel_value(&self) -> u32
    {
        u32::MIN
    }
}
