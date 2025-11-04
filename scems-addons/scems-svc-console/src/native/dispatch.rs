use core::cell::RefCell;

use alloc::vec::Vec;
use scems::value::{ErrValue, RetValue};
use scems_mcu::uart::UartDevice;
use scems_os::events::IEvents;
use scems_os::OS;

use crate::native::cache::ConsoleCache;
use crate::NativeConsoleCommandsExecute;
use crate::NativeConsoleCommandsParser;

const EVT_CMD_RX: u32 = 0x01;

pub struct ConsoleCommandsDispatchCore<O>
where
    O: OS,
{
    cache: RefCell<ConsoleCache>,
    executor_queue: Vec<Option<&'static dyn NativeConsoleCommandsExecute>>,
    dispatch_event: O::Events,
}

impl<O> ConsoleCommandsDispatchCore<O>
where
    O: OS,
{
    pub fn new(event: O::Events) -> Self
    {
        Self {
            cache: RefCell::new(ConsoleCache::new()),
            executor_queue: Vec::new(),
            dispatch_event: event,
        }
    }

    pub fn submit_executor(
        &mut self, exe: &'static dyn NativeConsoleCommandsExecute,
    ) -> RetValue<()>
    {
        for exec in self.executor_queue.iter_mut()
        {
            if let None = *exec
            {
                *exec = Some(exe);
                return Ok(());
            }
        }

        Err(ErrValue::StackOverflow)
    }

    pub fn dispatch(&mut self, serial_port: &UartDevice) -> RetValue<()>
    {
        serial_port.as_ref().async_receive(self.cache.borrow_mut().as_mut_bytes())?;
        self.dispatch_event.receive(EVT_CMD_RX, O::WAIT_FOREVER).or(Err(ErrValue::Overtime))?;

        let cache = self.cache.borrow();
        let mut commands = NativeConsoleCommandsParser::new(cache.as_bytes());
        let executor = commands.next()?;

        for slot in self.executor_queue.iter()
        {
            if let Some(x) = slot
            {
                x.name().as_bytes().eq(executor).then(|| x.execute_commands(&mut commands));
            }
        }

        Ok(())
    }

    pub fn set_dispatch_signal(&self, len: usize)
    {
        if len > usize::MIN
        {
            if let Ok(mut cache) = self.cache.try_borrow_mut()
            {
                cache.set_length(len);

                #[allow(unused_must_use)]
                self.dispatch_event.launch(EVT_CMD_RX);
            }
        }
    }
}
