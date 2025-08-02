use crate::common::result::RetValue;

use super::EventLaunch;

pub trait Uart
where
    Self: EventLaunch<dyn UartEventAgent>,
{
    fn transmit(&self, data: &[u8], timeout: u32) -> RetValue<()>;
    fn receive(&self, data: &mut [u8], timeout: u32) -> RetValue<u32>;
    fn receive_size(&self, data: &mut [u8], timeout: u32) -> RetValue<()>;
    fn async_transmit(&self, data: &[u8]) -> RetValue<()>;
    fn async_receive(&self, data: &mut [u8]) -> RetValue<()>;
    fn async_receive_size(&self, data: &mut [u8]) -> RetValue<()>;
    fn abort(&self) -> RetValue<()>;
}

pub trait UartEvent
{
    fn on_uart_tx_complete(&mut self) {}
    fn on_uart_rx_complete(&mut self, _size: u32) {}
    fn on_uart_rx_size_complete(&mut self) {}
    fn on_uart_abort_complete(&mut self) {}
    fn on_uart_error(&mut self) {}
}

pub trait UartEventAgent
{
    fn on_uart_tx_complete(&self) {}
    fn on_uart_rx_complete(&self, _size: u32) {}
    fn on_uart_rx_size_complete(&self) {}
    fn on_uart_abort_complete(&self) {}
    fn on_uart_error(&self) {}
}
