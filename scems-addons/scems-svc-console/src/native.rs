use log::Log;
use scems::value::RetValue;
use scems_mcu::uart::{UartCtrl, UartCtrlEvent, UartDevice};
use scems_os::task::ITaskMain;
use scems_os::OS;

use crate::native::dispatch::ConsoleDispatchCore;
use crate::native::print::ConsolePrintCore;
use crate::{Console, ConsoleExecute};

mod cache;
mod dispatch;
mod print;

pub struct NativeConsole<O>
where
    O: OS,
{
    serial_port: UartDevice,
    dispatcher: ConsoleDispatchCore<O>,
    printer: ConsolePrintCore<O>,
}

impl<O> NativeConsole<O>
where
    O: OS,
{
    pub fn new(
        uart: &'static mut dyn UartCtrl, events: O::Events, mutex1: O::Mutex, mutex2: O::Mutex,
    ) -> Self
    {
        Self {
            serial_port: UartDevice::new(uart),
            dispatcher: ConsoleDispatchCore::new(events, mutex1),
            printer: ConsolePrintCore::new(mutex2),
        }
    }
}

impl<O> Console for NativeConsole<O>
where
    O: OS,
{
    fn accept_dispatch(&self, exe: &'static dyn ConsoleExecute) -> RetValue<()>
    {
        self.dispatcher.accept_dispatch(exe)
    }
}

impl<O> ITaskMain for NativeConsole<O>
where
    O: Sized + OS,
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

impl<O> UartCtrlEvent for NativeConsole<O>
where
    O: Sized + OS,
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

unsafe impl<O> Send for NativeConsole<O> where O: OS {}

unsafe impl<O> Sync for NativeConsole<O> where O: OS {}

impl<O> Log for NativeConsole<O>
where
    O: OS,
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
