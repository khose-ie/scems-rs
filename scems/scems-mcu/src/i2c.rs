use scems::value::RetValue;
use scems_derive::{EnumAsU16, EnumAsU8};

use super::EventLaunch;

pub trait I2cMemCtrl
where
    Self: EventLaunch<dyn I2cMemCtrlEvent>,
{
    fn mem_write(
        &self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &[u8], timeout: u32,
    ) -> RetValue<()>;

    fn mem_read(
        &self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &mut [u8], timeout: u32,
    ) -> RetValue<()>;

    fn async_mem_write(
        &self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &[u8],
    ) -> RetValue<()>;

    fn async_mem_read(
        &self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &mut [u8],
    ) -> RetValue<()>;
}

pub trait I2cMemCtrlEvent
{
    fn on_i2c_mem_write_complete(&self) {}
    fn on_i2c_mem_read_complete(&self) {}
    fn on_i2c_mem_error(&self) {}
}

#[repr(u16)]
#[derive(PartialEq, Eq, Clone, Copy, EnumAsU16)]
pub enum I2cMemWide
{
    Bit8 = 0,
    Bit16 = 1,
}

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy, EnumAsU8)]
pub enum I2cDirection
{
    Receive = 0,
    Transmit = 1,
}

pub trait I2cMasterCtrl
where
    Self: EventLaunch<dyn I2cMasterCtrlEvent>,
{
    fn transmit(&self, saddr: u16, data: &[u8], timeout: u32) -> RetValue<()>;
    fn receive(&self, saddr: u16, data: &mut [u8], timeout: u32) -> RetValue<()>;
    fn async_transmit(&self, saddr: u16, data: &[u8]) -> RetValue<()>;
    fn async_receive(&self, saddr: u16, data: &mut [u8]) -> RetValue<()>;
}

pub trait I2cMasterCtrlEvent
{
    fn on_i2c_master_tx_complete(&self) {}
    fn on_i2c_master_rx_complete(&self) {}
    fn on_i2c_master_error(&self) {}
}

pub trait I2cSlaveCtrl
where
    Self: EventLaunch<dyn I2cSlaveCtrlEvent>,
{
    fn listen(&self) -> RetValue<()>;
    fn transmit(&self, data: &[u8], timeout: u32) -> RetValue<()>;
    fn receive(&self, data: &mut [u8], timeout: u32) -> RetValue<()>;
    fn async_transmit(&self, data: &[u8]) -> RetValue<()>;
    fn async_receive(&self, data: &mut [u8]) -> RetValue<()>;
}

pub trait I2cSlaveCtrlEvent
{
    fn on_i2c_slave_tx_complete(&self) {}
    fn on_i2c_slave_rx_complete(&self) {}
    fn on_i2c_slave_selected(&self, _direction: I2cDirection, _address: u16) {}
    fn on_i2c_slave_listen_complete(&self) {}
    fn on_i2c_slave_error(&self) {}
}

pub struct I2cMemDevice
{
    instance: *mut dyn I2cMemCtrl,
}

impl I2cMemDevice
{
    pub const fn new(instance: &'static mut dyn I2cMemCtrl) -> Self
    {
        Self { instance }
    }
}

impl AsRef<dyn I2cMemCtrl> for I2cMemDevice
{
    fn as_ref(&self) -> &'static dyn I2cMemCtrl
    {
        unsafe { &*self.instance }
    }
}

impl AsMut<dyn I2cMemCtrl> for I2cMemDevice
{
    fn as_mut(&mut self) -> &'static mut dyn I2cMemCtrl
    {
        unsafe { &mut *self.instance }
    }
}

pub struct I2cMasterDevice
{
    instance: *mut dyn I2cMasterCtrl,
}

impl I2cMasterDevice
{
    pub const fn new(instance: &'static mut dyn I2cMasterCtrl) -> Self
    {
        Self { instance }
    }
}

impl AsRef<dyn I2cMasterCtrl> for I2cMasterDevice
{
    fn as_ref(&self) -> &'static dyn I2cMasterCtrl
    {
        unsafe { &*self.instance }
    }
}

impl AsMut<dyn I2cMasterCtrl> for I2cMasterDevice
{
    fn as_mut(&mut self) -> &'static mut dyn I2cMasterCtrl
    {
        unsafe { &mut *self.instance }
    }
}

pub struct I2cSlaveDevice
{
    instance: *mut dyn I2cSlaveCtrl,
}

impl I2cSlaveDevice
{
    pub const fn new(instance: &'static mut dyn I2cSlaveCtrl) -> Self
    {
        Self { instance }
    }
}

impl AsRef<dyn I2cSlaveCtrl> for I2cSlaveDevice
{
    fn as_ref(&self) -> &'static dyn I2cSlaveCtrl
    {
        unsafe { &*self.instance }
    }
}

impl AsMut<dyn I2cSlaveCtrl> for I2cSlaveDevice
{
    fn as_mut(&mut self) -> &'static mut dyn I2cSlaveCtrl
    {
        unsafe { &mut *self.instance }
    }
}
