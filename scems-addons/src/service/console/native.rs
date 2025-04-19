use core::fmt::{Arguments, Write};

use scems::common::result::{IError, IResult};
use scems::os::common::mem::IMemBlock;
use scems::os::vendors::mem::{MemBlock, MemCache};

use crate::service::console::dispartches::ConsoleServiceDispatches;
use crate::service::console::terminal::{ConsoleServiceTerminal, SerialTerminal};
use crate::service::console::{ConsoleCommandExecution, ConsoleTerminal};

pub struct ConsoleServiceNative<'a>
{
    cache: MemCache<256>,
    dispatches: ConsoleServiceDispatches<'a>,
    terminal: MemBlock<ConsoleServiceTerminal>,
}

impl<'a> ConsoleServiceNative<'a>
{
    pub fn new() -> IResult<Self>
    {
        Ok(Self {
            cache: MemCache::new()?,
            dispatches: ConsoleServiceDispatches::new()?,
            terminal: MemBlock::from(ConsoleServiceTerminal::None)?,
        })
    }

    pub fn tick(&mut self)
    {
        self.terminal.tick(&self.dispatches);
    }

    pub fn assign_serial_terminal(&mut self, serial_terminal: SerialTerminal) -> IResult<()>
    {
        match *self.terminal
        {
            ConsoleServiceTerminal::None => self.terminal.set(ConsoleServiceTerminal::Serial(serial_terminal)),
            _ => return Err(IError::StackOverflow),
        }

        Ok(())
    }

    #[inline]
    pub fn unassign_terminal(&mut self)
    {
        self.terminal.set(ConsoleServiceTerminal::None);
    }

    #[inline]
    pub fn assign_command_execution(&mut self, execution: &'a dyn ConsoleCommandExecution) -> IResult<usize>
    {
        self.dispatches.assign_command_execution(execution)
    }

    #[inline]
    pub fn remove_command_execution(&mut self, execution: &'a dyn ConsoleCommandExecution)
    {
        self.dispatches.remove_command_execution(execution);
    }

    pub fn write_content(&mut self, _args: Arguments)
    {
        let mut cursor = BufWriter::new(self.cache.as_mut());
        let _ = write!(cursor, "{}", _args);
        cursor.add_end();
        let content = core::str::from_utf8(cursor.as_bytes()).unwrap_or("<invalid utf8>");

        self.terminal.write_content(content);
    }
}

pub struct BufWriter<'a>
{
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> BufWriter<'a>
{
    pub fn new(buf: &'a mut [u8]) -> Self
    {
        Self { buf, pos: 0 }
    }

    pub fn as_bytes(&self) -> &[u8]
    {
        &self.buf[..self.pos]
    }

    pub fn add_end(&mut self)
    {
        if self.pos >= self.buf.len() - 1
        {
            self.pos = self.buf.len() - 2;
        }

        self.buf[self.pos] = b'\r';
        self.pos += 1;
        self.buf[self.pos] = b'\n';
        self.pos += 1;
    }
}

impl<'a> Write for BufWriter<'a>
{
    fn write_str(&mut self, s: &str) -> core::fmt::Result
    {
        let bytes = s.as_bytes();

        if self.pos + bytes.len() > self.buf.len()
        {
            return Err(core::fmt::Error);
        }

        self.buf[self.pos..self.pos + bytes.len()].copy_from_slice(bytes);
        self.pos += bytes.len();

        Ok(())
    }
}
