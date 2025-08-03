//! Provide a common trait to operate the Universal Asynchronous Receiver Transmitter (UART).

use super::EventLaunch;
use crate::common::result::RetValue;

pub type UartDevice = &'static mut dyn UartCtrl;

/// A common trait to control UART peripheral, with functions to let the UART to do 
/// some basic actions.
/// 
/// The UART implementations for every MCU manufacturers should implement this trait.
/// And every upper modules who want to control an UART should reference this trait, and call the
/// functions of this trait.
pub trait UartCtrl
where
    Self: EventLaunch<dyn UartDeviceEventAgent>,
{
    /// Transmit the specified length data via UART.
    /// 
    /// This function will transmit all values in `data`, a slice that you transports, and return 
    /// the result after the transmition. It means that during the transmission, the function will 
    /// not return and the system will hung here, until the transmit end.
    /// The count of transmited data is specified by the length of the slice.
    /// 
    /// You could input a timeout value, to avoid the transmiting cost such a long time.
    /// If you want to wait until the transmission end, you should set `timeout` with `0xFFFFFFFF`.
    fn transmit(&self, data: &[u8], timeout: u32) -> RetValue<()>;

    /// Receive some data until the UART to be idle.
    /// 
    /// This function is used to receive a couple of data, it will end when the UART peripheral 
    /// come back to the "idle" state, it usually means that the remote device stop transmit data.
    /// This function always be used to the scenario that we don't know the length the remote want 
    /// to transmit to us, so, we receive until it doesn't transmit.
    /// So, this function will return an u32 value to tell the caller how many data it has been 
    /// received.
    /// Please attention, the input slice `data` also has a length, the max received data will not 
    /// over the length of `data`, when the receive data count over the length of `data`, whether 
    /// the UART is idle, the function will stop receiving and return.
    /// 
    /// You could input a timeout value, to avoid to hung the system a long time if the remote 
    /// doesn't transmit data in a long time.
    /// If you want to wait until and hung until receive something, you should set `timeout` with 
    /// `0xFFFFFFFF`.
    fn receive(&self, data: &mut [u8], timeout: u32) -> RetValue<u32>;

    /// Receive the specified length data via UART.
    /// 
    /// This function is used to receive the specified length data, it means that we don't care 
    /// whether the UART is idle, we only focus on the number.
    /// This function will not return until it receives the specified length data, if the UART 
    /// don't send enough before it come back to idle, the function will wait, and not return.
    /// The specified length is the length of input slice `data`.
    ///
    /// You could input a timeout value, to avoid to hung the system a long time if the remote 
    /// doesn't transmit enough data in a long time.
    /// If you want to wait until and hung until receive something, you should set `timeout` with 
    /// `0xFFFFFFFF`.
    fn receive_size(&self, data: &mut [u8], timeout: u32) -> RetValue<()>;

    /// Transmit the specified length data via UART in asynchronous mode.
    /// 
    /// The usage of this function is same with [`UartCtrl::transmit`], the only difference is 
    /// that this function will not hung. It will transmit the data via an asynchronous way like 
    /// the DMA and interrupts.
    /// So, the OK return of this function only means that the data has been moved to the buffer 
    /// of a DMA or interrupt, not means they are transmited successfully.
    /// 
    /// The result of the transmission will be notified via the 
    /// [`UartDeviceEventAgent::on_uart_tx_complete`].
    /// Use this function means that you have enable the DMA or interrupt transmission of this 
    /// DMA in the initialization code.
    fn async_transmit(&self, data: &[u8]) -> RetValue<()>;

    /// Receive some data until the UART to be idle.
    /// 
    /// The usage of this function is same with [`UartCtrl::receive`], the only difference is 
    /// that this function will not hung. It will receive the data via an asynchronous way like 
    /// the DMA and interrupts.
    /// So you need to input a `data` slice as the buffer space, the received data will be move 
    /// to the buffer, and the event [`UartDeviceEventAgent::on_uart_rx_complete`] will be call 
    /// when the UART come back to idle.
    /// Use this function means that you have enable the DMA or interrupt transmission of this 
    /// DMA in the initialization code.
    fn async_receive(&self, data: &mut [u8]) -> RetValue<()>;

    /// Receive some data until the UART to be idle.
    /// 
    /// The usage of this function is same with [`UartCtrl::receive_size`], the only difference 
    /// is that this function will not hung. It will receive the data via an asynchronous way 
    /// like the DMA and interrupts.
    /// So you need to input a `data` slice as the buffer space, the received data will be move 
    /// to the buffer, and the event [`UartDeviceEventAgent::on_uart_rx_size_complete`] will be 
    /// call when the UART come back to idle.
    /// Use this function means that you have enable the DMA or interrupt transmission of this 
    /// DMA in the initialization code.
    fn async_receive_size(&self, data: &mut [u8]) -> RetValue<()>;

    /// Abort all asynchronous actions.
    /// 
    /// Use this function to abort all asynchronous transmit or receive actions.
    fn abort(&self) -> RetValue<()>;
}

/// `UartEventAgent` as the meanings of the word, it is an agent, or the real handler, whatever, 
/// to handle all events sent from the `UartCtrl`.
/// 
/// Actually, these callback functions will be called in the interrupt vector handle which 
/// triggered by UART peripheral.
/// Please attention, don't use any waiting and hang actions in these functions, because it will 
/// block the system interrupts.
/// 
/// All functions of this trait have an empty default implementation, it meanus that you can only 
/// implement the function that you care about.
pub trait UartDeviceEventAgent
{
    /// This function will call when the asynchronous transmit has been completed.
    fn on_uart_tx_complete(&self) {}

    /// This function will call when you received something and it has been move to the buffer 
    /// `data` that you input in [`UartCtrl::async_receive`].
    /// 
    /// The receive length will be transport via parameter `_size`.
    /// Please attention, if the received data length is over the length of slice `data`, this 
    /// function will also be called and the `_size` is the length of slice `data`, the overed 
    /// data will be drop.
    fn on_uart_rx_complete(&self, _size: u32) {}

    /// This function will call when you received specified length of data and it has been move to the buffer 
    /// `data` that you input in [`UartCtrl::async_receive_size`].
    fn on_uart_rx_size_complete(&self) {}

    /// This function will call when your abort action has been completed.
    fn on_uart_abort_complete(&self) {}

    /// This function will be called when the UART peripheral has some errors.
    fn on_uart_error(&self) {}
}
