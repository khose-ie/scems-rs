pub mod adc;
pub mod can;
pub mod flash;
pub mod i2c;
pub mod io;
pub mod spi;
pub mod tim;
pub mod uart;
pub mod wd;

use crate::common::result::IResult;

/// Trait for one MCU chip.
///
/// All types in this trait has combined with peripheral trait.
/// So, when you want to define a components with some peripherals, you can only set this trait as
/// the only one trait bound.
pub trait Mcu
{
    type Adc: adc::Adc;
    type Can: can::Can;
    type Flash: flash::Flash;
    type I2cMaster: i2c::I2cMaster;
    type I2cMem: i2c::I2cMem;
    type I2cSlave: i2c::I2cSlave;
    type Io: io::Io;
    type Spi: spi::Spi;
    type Uart: uart::Uart;
    type TimBase: tim::TimBase;
    type TImPwm: tim::TimPwm;
    type WatchDog: wd::WatchDog;
}

pub trait EventLaunch<T: ?Sized>
{
    fn set_event_agent(&mut self, event_handle: &T) -> IResult<()>;
    fn clean_event_agent(&mut self);
}

pub trait AsHandlePtr<T: ?Sized>
{
    fn as_handle_ptr(&self) -> *mut T;
}

pub trait HandlePtr<T: ?Sized>
{
    fn handle_ptr(&self) -> *mut T;
}
