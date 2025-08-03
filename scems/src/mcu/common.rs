pub mod adc;
pub mod can;
pub mod flash;
pub mod i2c;
pub mod io;
pub mod spi;
pub mod tim;
pub mod uart;
pub mod wd;

/// Trait for one MCU chip.
///
/// All types in this trait has combined with peripheral trait.
/// So, when you want to define a components with some peripherals, you can only set this trait as
/// the only one trait bound.
pub trait Mcu
{
    type Adc: adc::AdcCtrl;
    type Can: can::CanCtrl;
    type Flash: flash::FlashCtrl;
    type I2cMaster: i2c::I2cMasterCtrl;
    type I2cMem: i2c::I2cMemCtrl;
    type I2cSlave: i2c::I2cSlaveCtrl;
    type Io: io::IoCtrl;
    type Spi: spi::SpiCtrl;
    type Uart: uart::UartCtrl;
    type TimBase: tim::TimBaseCtrl;
    type TImPwm: tim::TimPwmCtrl;
    type WatchDog: wd::WatchDogCtrl;
}

/// `EventLaunch` is a trait that the peripheral trait who implements this trait means that it can 
/// be set an event agent, and the peripheral will call function to send the event.
/// 
/// 
pub trait EventLaunch<T: ?Sized>
{
    fn set_event_agent(&mut self, event_handle: &T);
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
