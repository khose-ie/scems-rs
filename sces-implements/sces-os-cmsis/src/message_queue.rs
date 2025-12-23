use core::ops::Not;
use core::ptr::null;

use sces::value::{ErrValue, RetValue};
use sces::os::message_queue::{IMessageQueue, MessageContent};

use crate::native::*;

pub struct MessageQueue
{
    handle: osMessageQueueId_t,
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
    fn new(message_size: u32, message_count: u32) -> RetValue<Self>
    {
        let handle = unsafe { osMessageQueueNew(message_count, message_size, null()) };
        handle.is_null().not().then_some(handle).ok_or(ErrValue::InstanceCreateFailure)?;
        Ok(MessageQueue { handle })
    }

    fn send(&self, content: &dyn MessageContent, timeout: u32) -> RetValue<()>
    {
        unsafe { osMessageQueuePut(self.handle, content.as_ptr(), 0, timeout).into() }
    }

    fn receive(&self, cache: &mut dyn MessageContent, timeout: u32) -> RetValue<()>
    {
        let mut prio: u8 = 0;
        unsafe { osMessageQueueGet(self.handle, cache.as_mut_ptr(), &mut prio, timeout).into() }
    }
}
