use crate::common::result::IResult;

use super::{AsEventPtr, EventHandle};

pub trait Spi
where
    Self: EventHandle<dyn SpiEventPtr>,
{
    fn transmit(&self, data: &[u8], timeout: u32) -> IResult<()>;
    fn receive(&self, data: &mut [u8], timeout: u32) -> IResult<()>;
    fn transmit_receive(&self, tx_data: &[u8], rx_data: &mut [u8], timeout: u32) -> IResult<()>;

    fn async_transmit(&self, data: &[u8]) -> IResult<()>;
    fn async_receive(&self, data: &mut [u8]) -> IResult<()>;
    fn async_transmit_receive(&self, tx_data: &[u8], rx_data: &mut [u8]) -> IResult<()>;

    fn abort(&self) -> IResult<()>;
}

pub trait SpiEvent
{
    fn on_spi_tx_complete(&mut self) {}
    fn on_spi_rx_complete(&mut self) {}
    fn on_spi_tx_rx_complete(&mut self) {}
    fn on_spi_abort_complete(&mut self) {}
    fn on_spi_error(&mut self) {}
}

pub trait SpiEventPtr
where
    Self: SpiEvent + AsEventPtr<dyn SpiEvent>,
{
}
