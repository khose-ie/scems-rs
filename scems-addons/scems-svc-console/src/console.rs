use log::{LevelFilter, Log};
use scems::value::{ErrValue, RetValue};
use scems_mcu::uart::{UartCtrl, UartCtrlEvent, UartDevice};
use scems_os::mutex::MutexSample;
use scems_os::task::ITaskMain;
use scems_os::OS;

use crate::console::dispatch::ConsoleCommandsDispatchCore;
use crate::console::print::ConsolePrintCore;

mod cache;
mod dispatch;
mod print;

pub struct ConsoleService<O>
where
    O: OS,
{
    serial_port: UartDevice,
    dispatcher: ConsoleCommandsDispatchCore<O>,
    printer: MutexSample<O::Mutex, ConsolePrintCore>,
}

impl<O> ConsoleService<O>
where
    O: OS,
{
    pub fn new(uart: &'static mut dyn UartCtrl, events: O::Events, mutex: O::Mutex) -> Self
    {
        Self {
            serial_port: UartDevice::new(uart),
            dispatcher: ConsoleCommandsDispatchCore::new(events),
            printer: MutexSample::new(mutex, ConsolePrintCore::new()),
        }
    }

    pub fn initialize(&'static self, level: LevelFilter) -> RetValue<()>
    {
        log::set_max_level(level);
        log::set_logger(self).or(Err(ErrValue::InstanceDuplicate))
    }

    pub fn submit_commands_executor(
        &mut self, exe: &'static dyn ConsoleCommandsExecute,
    ) -> RetValue<()>
    {
        self.dispatcher.submit_executor(exe)
    }
}

impl<O> ITaskMain for ConsoleService<O>
where
    O: Sized + OS,
{
    fn main(&mut self)
    {
        loop
        {
            #[allow(unused_must_use)]
            self.dispatcher.dispatch(&self.serial_port);
        }
    }
}

impl<O> UartCtrlEvent for ConsoleService<O>
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

unsafe impl<O> Send for ConsoleService<O> where O: OS {}

unsafe impl<O> Sync for ConsoleService<O> where O: OS {}

impl<O> Log for ConsoleService<O>
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
        self.printer.attempt_lock(100, |core| core.writes(&self.serial_port, record));
    }

    fn flush(&self) {}
}

pub struct ConsoleCommandsParser<'a>
{
    cmds: &'a [u8],
    position: usize,
}

impl<'a> ConsoleCommandsParser<'a>
{
    pub fn new(cmds: &'a [u8]) -> Self
    {
        Self { cmds, position: 0 }
    }

    pub fn next(&mut self) -> RetValue<&'a [u8]>
    {
        let mut pos = self.position;

        while pos < self.cmds.len()
            && (self.cmds[pos] == b' ' || self.cmds[pos] == b'\r' || self.cmds[pos] == b'\n')
        {
            pos += 1;
        }

        if pos >= self.cmds.len() as usize
        {
            return Err(ErrValue::StackOverflow);
        }

        let start = pos;

        while pos < self.cmds.len()
            && self.cmds[pos] != b' '
            && self.cmds[pos] != b'\r'
            && self.cmds[pos] != b'\n'
        {
            pos += 1;
        }

        self.position = pos;

        Ok(&self.cmds[start..pos])
    }

    pub fn residual(&self) -> &[u8]
    {
        &self.cmds[self.position..]
    }
}

pub trait ConsoleCommandsExecute
{
    fn name(&self) -> &str;
    fn execute_commands(&self, commands: &mut ConsoleCommandsParser);
}
