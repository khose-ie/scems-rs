use core::fmt::Arguments;

#[derive(PartialEq, PartialOrd)]
pub enum LogLevel
{
    Error,
    Warn,
    Info,
    Debug,
}

pub trait LogStream
{
    fn write_content(&self, _args: Arguments) {}
    fn as_log_stream(&self) -> &dyn LogStream;
}

static mut WRITER: LogWriter = LogWriter::new();

#[allow(static_mut_refs)]
pub fn assign_stream(stream: &'static dyn LogStream)
{
    unsafe { WRITER.assign_stream(stream) };
}

#[allow(static_mut_refs)]
pub fn format_write(level: LogLevel, args: Arguments)
{
    unsafe { WRITER.format_write(level, args) };
}

#[allow(static_mut_refs)]
pub fn set_level(level: LogLevel)
{
    unsafe { WRITER.set_level(level) };
}

struct LogWriter
{
    level: LogLevel,
    stream: Option<&'static dyn LogStream>,
}

impl LogWriter
{
    pub const fn new() -> Self
    {
        Self { level: LogLevel::Warn, stream: None }
    }

    pub fn assign_stream(&mut self, stream: &'static dyn LogStream)
    {
        self.stream = Some(stream);
    }

    pub fn format_write(&mut self, level: LogLevel, args: Arguments)
    {
        if level < self.level
        {
            if let Some(stream) = self.stream
            {
                stream.write_content(args);
            }
        }
    }

    pub fn set_level(&mut self, level: LogLevel)
    {
        self.level = level;
    }
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {{
        scems::common::log::format_write($level, core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        scems::log!(scems::common::log::LogLevel::Error, $($arg)*);
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        scems::log!(scems::common::log::LogLevel::Warn, $($arg)*);
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        scems::log!(scems::common::log::LogLevel::Info, $($arg)*);
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        scems::log!(scems::common::log::LogLevel::Debug, $($arg)*);
    };
}
