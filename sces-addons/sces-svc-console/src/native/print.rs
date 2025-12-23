use core::fmt::Write;

use sces::value::{ErrValue, RetValue};
use sces::mcu::uart::UartDevice;
use sces::os::{mutex::MutexSample, RTOS};

use crate::native::cache::ConsoleCache;

pub struct ConsolePrintCore<OS>
where
    OS: RTOS,
{
    cache: MutexSample<OS, ConsoleCache>,
}

impl<OS> ConsolePrintCore<OS>
where
    OS: RTOS,
{
    pub fn new() -> RetValue<Self>
    {
        Ok(Self { cache: MutexSample::new(ConsoleCache::new())? })
    }

    pub fn writes(&self, serial_port: &UartDevice, record: &log::Record) -> RetValue<()>
    {
        if let Ok(mut x) = self.cache.attempt_lock()
        {
            x.clean();

            #[allow(unused_must_use)]
            writeln!(x, "[{:>5}] {}", record.level(), record.args())
                .map_err(|_| ErrValue::FormatFailure)
                .and_then(|()| serial_port.as_ref().transmit(x.as_bytes(), 100));
        }

        Ok(())
    }
}
