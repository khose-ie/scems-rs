use core::{ops::Not, ptr::null};

use sces::value::{ErrValue, RetValue};
use sces_os::events::IEvents;

use crate::native::*;

pub struct Events
{
    handle: osEventFlagsId_t,
}

impl Drop for Events
{
    fn drop(&mut self)
    {
        unsafe { osEventFlagsDelete(self.handle) };
    }
}

impl IEvents for Events
{
    fn new() -> RetValue<Self>
    {
        let handle = unsafe { osEventFlagsNew(null()) };
        handle.is_null().not().then_some(handle).ok_or(ErrValue::InstanceCreateFailure)?;
        Ok(Events { handle })
    }

    fn put(&self, events: u32) -> RetValue<()>
    {
        if events & osFlagsError != 0
        {
            return Err(ErrValue::Param);
        }

        let event_state = unsafe { osEventFlagsSet(self.handle, events) };

        if event_state & osFlagsError != 0
        {
            return osStatus_t::from(event_state as i32).into();
        }

        Ok(())
    }

    fn wait(&self, events: u32, timeout: u32) -> RetValue<u32>
    {
        let event_state = unsafe { osEventFlagsWait(self.handle, events, osFlagsWaitAny, timeout) };

        if event_state & osFlagsError != 0
        {
            return Err(osStatus_t::from(event_state as i32).into());
        }

        Ok(event_state)
    }
}
