use core::fmt::Write;

use scems::value::{ErrValue, RetValue};
use scems_mcu::uart::UartDevice;
use scems_os::{mutex::MutexSample, OS};

use crate::native::cache::ConsoleCache;

pub struct ConsolePrintCore<O>
where
    O: OS,
{
    cache: MutexSample<O::Mutex, ConsoleCache>,
}

impl<O> ConsolePrintCore<O>
where
    O: OS,
{
    pub fn new() -> RetValue<Self>
    {
        Ok(Self { cache: MutexSample::new(ConsoleCache::new())? })
    }

    #[rustfmt::skip]
    pub fn writes(&self, serial_port: &UartDevice, record: &log::Record) -> RetValue<()>
    {
        self.cache.lock_then_with(|x|
        { 
            x.clean();

            writeln!(x, "[{:>5}] {}", record.level(), record.args())
                .map_err(|_| ErrValue::FormatFaliure)
                .and_then(|()| serial_port.as_ref().transmit(x.as_bytes(), 100))
        })
    }
}
