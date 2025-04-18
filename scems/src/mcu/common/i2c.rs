use crate::derive::{EnumCastU16, EnumCastU8};
use crate::common::result::IResult;

use super::{AsEventPtr, EventHandle};

pub trait I2cMem
where
    Self: EventHandle<dyn I2cMemEventPtr>,
{
    fn mem_write(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &[u8], timeout: u32) -> IResult<()>;
    fn mem_read(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &mut [u8], timeout: u32) -> IResult<()>;
    fn async_mem_write(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &[u8]) -> IResult<()>;
    fn async_mem_read(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &mut [u8]) -> IResult<()>;
}

pub trait I2cMemEvent
{
    fn on_i2c_mem_write_complete(&mut self) {}
    fn on_i2c_mem_read_complete(&mut self) {}
    fn on_i2c_mem_error(&mut self) {}
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

pub trait I2cMemEventPtr
where
    Self: I2cMemEvent + AsEventPtr<dyn I2cMemEvent>,
{
}

pub trait I2cMaster
where
    Self: EventHandle<dyn I2cMasterEventPtr>,
{
    fn transmit(&self, saddr: u16, data: &[u8], timeout: u32) -> IResult<()>;
    fn receive(&self, saddr: u16, data: &mut [u8], timeout: u32) -> IResult<()>;
    fn async_transmit(&self, saddr: u16, data: &[u8]) -> IResult<()>;
    fn async_receive(&self, saddr: u16, data: &mut [u8]) -> IResult<()>;
}

pub trait I2cMasterEvent
{
    fn on_i2c_master_tx_complete(&mut self) {}
    fn on_i2c_master_rx_complete(&mut self) {}
    fn on_i2c_master_error(&mut self) {}
}

pub trait I2cMasterEventPtr
where
    Self: I2cMasterEvent + AsEventPtr<dyn I2cMasterEvent>,
{
}

pub trait I2cSlave
where
    Self: EventHandle<dyn I2cSlaveEventPtr>,
{
    fn listen(&self) -> IResult<()>;
    fn transmit(&self, data: &[u8], timeout: u32) -> IResult<()>;
    fn receive(&self, data: &mut [u8], timeout: u32) -> IResult<()>;
    fn async_transmit(&self, data: &[u8]) -> IResult<()>;
    fn async_receive(&self, data: &mut [u8]) -> IResult<()>;
}

pub trait I2cSlaveEvent
{
    fn on_i2c_slave_tx_complete(&mut self) {}
    fn on_i2c_slave_rx_complete(&mut self) {}
    fn on_i2c_slave_selected(&mut self, _direction: I2cDirection, _address: u16) {}
    fn on_i2c_slave_listen_complete(&mut self) {}
    fn on_i2c_slave_error(&mut self) {}
}

pub trait I2cSlaveEventPtr
where
    Self: I2cSlaveEvent + AsEventPtr<dyn I2cSlaveEvent>,
{
}
