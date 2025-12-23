use core::ptr::null;

use sces::value::{ErrValue, RetValue};
use sces_os::events::IEvents;

use crate::os::native::*;

pub struct Events
{
    handle: ScesEventHandle,
}

impl Drop for Events
{
    fn drop(&mut self)
    {
        unsafe { sces_event_delete(self.handle) };
    }
}

impl IEvents for Events
{
    fn new() -> RetValue<Self>
    {
        let handle = unsafe { sces_event_create(null()) };
        (!handle.is_null()).then_some(Events { handle }).ok_or(ErrValue::InstanceCreateFailure)
    }

    fn put(&self, events: u32) -> RetValue<()>
    {
        unsafe { sces_event_put(self.handle, events).into() }
    }

    fn wait(&self, events: u32, timeout: u32) -> RetValue<u32>
    {
        let waited_events = SCES_EVENT_NONE;

        unsafe {
            sces_event_wait_and_clear(
                self.handle,
                events,
                &waited_events as *const u32 as *mut u32,
                timeout,
            )
            .map(waited_events)
        }
    }
}
