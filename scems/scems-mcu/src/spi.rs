use super::EventLaunch;
use scems::value::RetValue;

pub type SpiDevice = &'static mut dyn SpiCtrl;

pub trait SpiCtrl
where
    Self: EventLaunch<dyn SpiCtrlEvent>,
{
    fn transmit(&self, data: &[u8], timeout: u32) -> RetValue<()>;

    fn receive(&self, data: &mut [u8], timeout: u32) -> RetValue<()>;

    fn transmit_receive(&self, tx_data: &[u8], rx_data: &mut [u8], timeout: u32) -> RetValue<()>;

    fn async_transmit(&self, data: &[u8]) -> RetValue<()>;

    fn async_receive(&self, data: &mut [u8]) -> RetValue<()>;

    fn async_transmit_receive(&self, tx_data: &[u8], rx_data: &mut [u8]) -> RetValue<()>;

    fn abort(&self) -> RetValue<()>;
}

pub trait SpiCtrlEvent
{
    fn on_spi_tx_complete(&self) {}
    fn on_spi_rx_complete(&self) {}
    fn on_spi_tx_rx_complete(&self) {}
    fn on_spi_abort_complete(&self) {}
    fn on_spi_error(&self) {}
}
