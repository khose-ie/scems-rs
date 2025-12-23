use core::cell::RefCell;

use alloc::vec::Vec;
use log::warn;
use sces::value::{ErrValue, RetValue};
use sces::mcu::uart::UartDevice;
use sces::os::events::IEvents;
use sces::os::mem::SafeVec;
use sces::os::mutex::MutexSample;
use sces::os::RTOS;

use crate::native::cache::ConsoleCache;
use crate::svc::CS;
use crate::ConsoleCommands;
use crate::ConsoleExecute;

const EVT_CMD_RX: u32 = 0x01;

pub struct ConsoleDispatchCore<OS>
where
    OS: RTOS,
{
    cache: RefCell<ConsoleCache>,
    exe_queue: MutexSample<OS, Vec<&'static dyn ConsoleExecute>>,
    dispatch_event: OS::Events,
}

impl<OS> ConsoleDispatchCore<OS>
where
    OS: RTOS,
{
    pub fn new() -> RetValue<Self>
    {
        Ok(Self {
            cache: RefCell::new(ConsoleCache::new()),
            exe_queue: MutexSample::new(Vec::new())?,
            dispatch_event: OS::Events::new()?,
        })
    }

    pub fn search_exe(
        queue: &mut Vec<&'static dyn ConsoleExecute>, exe_name: &[u8],
    ) -> Option<&'static dyn ConsoleExecute>
    {
        queue.iter().find(|x| x.exe_name().as_bytes().eq(exe_name)).map(|x| *x)
    }

    pub fn accept_dispatch(&self, exe: &'static dyn ConsoleExecute) -> RetValue<()>
    {
        self.exe_queue.attempt_lock_then(|x| x.attempt_push(exe))
    }

    pub fn wait_and_dispatch(&self, serial_port: &UartDevice) -> RetValue<()>
    {
        serial_port.as_ref().async_receive(self.cache.borrow_mut().as_bytes_mut())?;
        self.dispatch_event.wait(EVT_CMD_RX, OS::WAIT_MAX).or(Err(ErrValue::Timeout))?;

        let cache = self.cache.borrow();
        let mut commands = ConsoleCommands::new(cache.as_bytes());
        let exe_name = commands.next().ok_or(ErrValue::FormatFailure)?;

        self.exe_queue
            .attempt_lock_then(|x| Self::search_exe(x, exe_name).ok_or(ErrValue::InstanceNotFound))
            .and_then(|x| x.exe_with_cmds(&mut commands))
            .inspect_err(|_| warn!("{CS} Can't recognize the inputed command."))
    }

    pub fn set_dispatch_signal(&self, len: usize)
    {
        if len > usize::MIN
        {
            if let Ok(mut cache) = self.cache.try_borrow_mut()
            {
                cache.set_length(len);

                #[allow(unused_must_use)]
                self.dispatch_event.put(EVT_CMD_RX);
            }
        }
    }
}
