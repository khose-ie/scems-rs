use core::fmt::Write;

use scems::value::RetValue;
use scems_mcu::uart::UartDevice;

use crate::native::cache::ConsoleCache;

pub struct ConsolePrintCore
{
    cache: ConsoleCache,
}

impl ConsolePrintCore
{
    pub fn new() -> Self
    {
        Self { cache: ConsoleCache::new() }
    }

    pub fn writes(&mut self, serial_port: &UartDevice, record: &log::Record) -> RetValue<()>
    {
        self.cache.clean_up();

        if let Ok(()) = write!(self.cache, "[{:>5}] {}\n", record.level(), record.args())
        {
            #[allow(unused_must_use)]
            serial_port.as_ref().transmit(self.cache.as_bytes(), 100)?;
        }

        Ok(())
    }
}
