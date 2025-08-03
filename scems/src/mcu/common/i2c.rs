use crate::common::result::RetValue;
use crate::derive::{EnumCastU16, EnumCastU8};

use super::EventLaunch;

pub trait I2cMemDevice
where
    Self: EventLaunch<dyn I2cMemDeviceEventAgent>,
{
    fn mem_write(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &[u8], timeout: u32) -> RetValue<()>;
    fn mem_read(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &mut [u8], timeout: u32) -> RetValue<()>;
    fn async_mem_write(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &[u8]) -> RetValue<()>;
    fn async_mem_read(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &mut [u8]) -> RetValue<()>;
}

pub trait I2cMemEvent
{
    fn on_i2c_mem_write_complete(&mut self) {}
    fn on_i2c_mem_read_complete(&mut self) {}
    fn on_i2c_mem_error(&mut self) {}
}

pub trait I2cMemDeviceEventAgent
{
    fn on_i2c_mem_write_complete(&self) {}
    fn on_i2c_mem_read_complete(&self) {}
    fn on_i2c_mem_error(&self) {}
}

#[repr(u16)]
#[derive(PartialEq, Eq, Clone, Copy, EnumCastU16)]
pub enum I2cMemWide
{
    Bit8 = 0,

    Bit16 = 1,
}

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy, EnumCastU8)]
pub enum I2cDirection
{
    Receive = 0,
    Transmit = 1,
}

pub trait I2cMasterDevice
where
    Self: EventLaunch<dyn I2cMasterDeviceEventAgent>,
{
    fn transmit(&self, saddr: u16, data: &[u8], timeout: u32) -> RetValue<()>;
    fn receive(&self, saddr: u16, data: &mut [u8], timeout: u32) -> RetValue<()>;
    fn async_transmit(&self, saddr: u16, data: &[u8]) -> RetValue<()>;
    fn async_receive(&self, saddr: u16, data: &mut [u8]) -> RetValue<()>;
}

pub trait I2cMasterEvent
{
    fn on_i2c_master_tx_complete(&mut self) {}
    fn on_i2c_master_rx_complete(&mut self) {}
    fn on_i2c_master_error(&mut self) {}
}

pub trait I2cMasterDeviceEventAgent
{
    fn on_i2c_master_tx_complete(&self) {}
    fn on_i2c_master_rx_complete(&self) {}
    fn on_i2c_master_error(&self) {}
}

pub trait I2cSlaveDevice
where
    Self: EventLaunch<dyn I2cSlaveDeviceEventAgent>,
{
    fn listen(&self) -> RetValue<()>;
    fn transmit(&self, data: &[u8], timeout: u32) -> RetValue<()>;
    fn receive(&self, data: &mut [u8], timeout: u32) -> RetValue<()>;
    fn async_transmit(&self, data: &[u8]) -> RetValue<()>;
    fn async_receive(&self, data: &mut [u8]) -> RetValue<()>;
}

pub trait I2cSlaveEvent
{
    fn on_i2c_slave_tx_complete(&mut self) {}
    fn on_i2c_slave_rx_complete(&mut self) {}
    fn on_i2c_slave_selected(&mut self, _direction: I2cDirection, _address: u16) {}
    fn on_i2c_slave_listen_complete(&mut self) {}
    fn on_i2c_slave_error(&mut self) {}
}

pub trait I2cSlaveDeviceEventAgent
{
    fn on_i2c_slave_tx_complete(&self) {}
    fn on_i2c_slave_rx_complete(&self) {}
    fn on_i2c_slave_selected(&self, _direction: I2cDirection, _address: u16) {}
    fn on_i2c_slave_listen_complete(&self) {}
    fn on_i2c_slave_error(&self) {}
}
