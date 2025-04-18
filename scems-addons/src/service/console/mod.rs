mod dispartches;
pub mod native;
pub mod terminal;

use core::fmt::Arguments;

use scems::common::log::LogStream;
use scems::common::result::IResult;
use scems::os::common::mutex::IMutexBlock;
use scems::os::common::task::TaskMain;
use scems::os::vendors::mutex::MutexBlock;
use scems::os::vendors::{delay, COMMON_TASK_TICK};

use crate::service::console::native::ConsoleServiceNative;
use crate::service::console::terminal::SerialTerminal;

pub trait Console<'a>
where
    Self: LogStream + ConsoleCommandStore<'a>,
{
    fn assign_serial_terminal(&self, serial_terminal: SerialTerminal) -> IResult<()>;
    fn unassign_terminal(&self);
}

pub trait ConsoleTerminal
{
    fn tick<'a>(&mut self, dispatches: &dyn ConsoleCommandDispatches);
    fn write_content(&self, _content: &str);
}

pub trait ConsoleCommandStore<'a>
{
    fn assign_command_execution(&self, execution: &'a dyn ConsoleCommandExecution) -> IResult<usize>;
    fn remove_command_execution(&self, execution: &'a dyn ConsoleCommandExecution);
}

pub trait ConsoleCommandDispatches
{
    fn dispatch_and_execute(&self, command: &[u8], response: &mut [u8]) -> IResult<()>;
}

pub trait ConsoleCommandExecution
{
    fn console_command_name(&self) -> &str;
    fn execute_console_command(&self, args: &[&[u8]], response: &mut [u8]) -> IResult<()>;
}

pub struct ConsoleService<'a>
{
    native: MutexBlock<ConsoleServiceNative<'a>>,
}

impl<'a> ConsoleService<'a>
{
    pub fn new() -> IResult<Self>
    {
        Ok(Self { native: MutexBlock::new(ConsoleServiceNative::new()?)? })
    }
}

impl<'a> Console<'a> for ConsoleService<'a>
{
    #[inline]
    fn assign_serial_terminal(&self, serial_terminal: SerialTerminal) -> IResult<()>
    {
        self.native.lock_with(|native| native.assign_serial_terminal(serial_terminal))
    }

    #[inline]
    fn unassign_terminal(&self)
    {
        self.native.lock(|native| native.unassign_terminal());
    }
}

impl<'a> ConsoleCommandStore<'a> for ConsoleService<'a>
{
    #[inline]
    fn assign_command_execution(&self, execution: &'a dyn ConsoleCommandExecution) -> IResult<usize>
    {
        self.native.lock_with(|native| native.assign_command_execution(execution))
    }

    #[inline]
    fn remove_command_execution(&self, execution: &'a dyn ConsoleCommandExecution)
    {
        self.native.lock(|native| native.remove_command_execution(execution));
    }
}

impl<'a> TaskMain for ConsoleService<'a>
{
    fn main(&mut self)
    {
        loop
        {
            self.native.lock(|native| native.tick());
            delay(COMMON_TASK_TICK);
        }
    }
}

impl<'a> LogStream for ConsoleService<'a>
{
    fn write_content(&self, _args: Arguments)
    {
        self.native.lock(|native| native.write_content(_args));
    }

    fn as_log_stream(&self) -> &dyn LogStream
    {
        self
    }
}
