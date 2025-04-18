use crate::common::result::IResult;

use super::{AsEventPtr, EventHandle};

pub trait Uart
where
    Self: EventHandle<dyn UartEventPtr>,
{
    fn transmit(&self, data: &[u8], timeout: u32) -> IResult<()>;
    fn receive(&self, data: &mut [u8], timeout: u32) -> IResult<u32>;
    fn receive_size(&self, data: &mut [u8], timeout: u32) -> IResult<()>;
    fn async_transmit(&self, data: &[u8]) -> IResult<()>;
    fn async_receive(&self, data: &mut [u8]) -> IResult<()>;
    fn async_receive_size(&self, data: &mut [u8]) -> IResult<()>;
    fn abort(&self) -> IResult<()>;
}

pub trait UartEvent
{
    fn on_uart_tx_complete(&mut self) {}
    fn on_uart_rx_complete(&mut self, _size: u32) {}
    fn on_uart_rx_size_complete(&mut self) {}
    fn on_uart_abort_complete(&mut self) {}
    fn on_uart_error(&mut self) {}
}

pub trait UartEventPtr
where
    Self: UartEvent + AsEventPtr<dyn UartEvent>,
{
}
