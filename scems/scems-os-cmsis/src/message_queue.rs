use core::ptr::null;
use core::{ffi::c_void, ops::Not};

use scems::value::{ErrValue, RetValue};
use scems_os::message_queue::{IMessageQueue, MessageContent};

use crate::native::*;

pub struct MessageQueue
{
    handle: osMessageQueueId_t,
}

impl MessageQueue
{
    pub fn new(message_count: u32, message_size: u32) -> RetValue<Self>
    {
        let handle = unsafe { osMessageQueueNew(message_count, message_size, null()) };
        handle.is_null().not().then_some(handle).ok_or(ErrValue::InstanceCreateFailure)?;
        Ok(MessageQueue { handle })
    }
}

impl Drop for MessageQueue
{
    fn drop(&mut self)
    {
        unsafe { osMessageQueueDelete(self.handle) };
    }
}

impl IMessageQueue for MessageQueue
{
    fn launch(&self, content: &dyn MessageContent, timeout: u32) -> RetValue<()>
    {
        unsafe {
            osMessageQueuePut(self.handle, content.as_ptr() as *mut c_void, 0, timeout).into()
        }
    }

    fn receive(&self, cache: &mut dyn MessageContent, timeout: u32) -> RetValue<()>
    {
        let mut prio: u8 = 0;
        unsafe {
            osMessageQueueGet(self.handle, cache.as_ptr() as *mut c_void, &mut prio, timeout).into()
        }
    }
}
