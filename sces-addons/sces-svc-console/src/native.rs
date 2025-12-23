use log::Log;
use sces::value::RetValue;
use sces::mcu::uart::{UartCtrl, UartCtrlEvent, UartDevice};
use sces::os::task::ITaskMain;
use sces::os::RTOS;

use crate::native::dispatch::ConsoleDispatchCore;
use crate::native::print::ConsolePrintCore;
use crate::{Console, ConsoleExecute};

mod cache;
mod dispatch;
mod print;

pub struct NativeConsole<OS>
where
    OS: RTOS,
{
    serial_port: UartDevice,
    dispatcher: ConsoleDispatchCore<OS>,
    printer: ConsolePrintCore<OS>,
}

impl<OS> NativeConsole<OS>
where
    OS: RTOS,
{
    pub fn new(uart: &'static mut dyn UartCtrl) -> RetValue<Self>
    {
        Ok(Self {
            serial_port: UartDevice::new(uart),
            dispatcher: ConsoleDispatchCore::new()?,
            printer: ConsolePrintCore::new()?,
        })
    }
}

impl<OS> Console for NativeConsole<OS>
where
    OS: RTOS,
{
    fn accept_dispatch(&self, exe: &'static dyn ConsoleExecute) -> RetValue<()>
    {
        self.dispatcher.accept_dispatch(exe)
    }
}

impl<OS> ITaskMain for NativeConsole<OS>
where
    OS: Sized + RTOS,
{
    fn main(&mut self)
    {
        loop
        {
            #[allow(unused_must_use)]
            self.dispatcher.wait_and_dispatch(&self.serial_port);
        }
    }
}

impl<OS> UartCtrlEvent for NativeConsole<OS>
where
    OS: Sized + RTOS,
{
    fn on_uart_rx_complete(&self, size: u32)
    {
        #[allow(unused_must_use)]
        self.dispatcher.set_dispatch_signal(size as usize);
    }

    fn on_uart_error(&self)
    {
        self.on_uart_rx_complete(0);
    }
}

unsafe impl<OS> Send for NativeConsole<OS> where OS: RTOS {}

unsafe impl<OS> Sync for NativeConsole<OS> where OS: RTOS {}

impl<OS> Log for NativeConsole<OS>
where
    OS: RTOS,
{
    #[allow(unused_variables)]
    fn enabled(&self, metadata: &log::Metadata) -> bool
    {
        true
    }

    fn log(&self, record: &log::Record)
    {
        #[allow(unused_must_use)]
        self.printer.writes(&self.serial_port, record);
    }

    fn flush(&self) {}
}
