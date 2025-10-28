use core::cell::RefCell;

use scems::value::{ErrValue, RetValue};
use scems_mcu::uart::UartDevice;
use scems_os::events::IEvents;
use scems_os::OS;

use crate::console::cache::ConsoleCache;
use crate::ConsoleCommandsExecute;
use crate::ConsoleCommandsParser;

const EVT_CMD_RX: u32 = 0x01;
const QUEUE_SIZE: usize = 16;

pub struct ConsoleCommandsDispatchCore<O>
where
    O: OS,
{
    cache: RefCell<ConsoleCache>,
    executor_queue: [Option<&'static dyn ConsoleCommandsExecute>; QUEUE_SIZE],
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
            executor_queue: [None; QUEUE_SIZE],
            dispatch_event: event,
        }
    }

    pub fn submit_executor(&mut self, exe: &'static dyn ConsoleCommandsExecute) -> RetValue<()>
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
        let mut commands = ConsoleCommandsParser::new(cache.as_bytes());
        let executor = commands.next()?;

        for slot in self.executor_queue
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
