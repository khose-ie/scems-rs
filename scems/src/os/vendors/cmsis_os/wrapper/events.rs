use core::ptr::null;

use crate::common::cast::CastOpt;
use crate::common::result::{IError, IResult};
use crate::os::common::events::IEvents;
use crate::os::vendors::cmsis_os::cmsis::*;

pub struct Events
{
    handle: osEventFlagsId_t,
}

impl Events
{
    #[rustfmt::skip]
    pub fn new() -> IResult<Self>
    {
        let handle = unsafe { osEventFlagsNew(null()).cast_opt().ok_or(IError::InstanceCreate) }?;
        Ok(Events { handle })
    }
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
    fn launch(&self, events: u32) -> IResult<()>
    {
        if events & osFlagsError != 0
        {
            return Err(IError::Param);
        }

        let event_state = unsafe { osEventFlagsSet(self.handle, events) };

        if event_state & osFlagsError != 0
        {
            return osStatus_t::from(event_state as i32).into();
        }

        Ok(())
    }

    fn receive(&self, events: u32, timeout: u32) -> IResult<u32>
    {
        let event_state = unsafe { osEventFlagsWait(self.handle, events, osFlagsWaitAny, timeout) };

        if event_state & osFlagsError != 0
        {
            return Err(osStatus_t::from(event_state as i32).into());
        }

        Ok(event_state)
    }
}
