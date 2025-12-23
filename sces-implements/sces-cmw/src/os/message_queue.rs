use core::ptr::null;

use sces::value::RetValue;
use sces::os::message_queue::{IMessageQueue, MessageContent};

use crate::os::native::*;

pub struct MessageQueue
{
    handle: ScesMessageQueueHandle,
}

impl Drop for MessageQueue
{
    fn drop(&mut self)
    {
        unsafe { sces_mq_delete(self.handle) };
    }
}

impl IMessageQueue for MessageQueue
{
    fn new(message_count: u32, message_size: u32) -> RetValue<Self>
    where
        Self: Sized,
    {
        let handle = unsafe { sces_mq_create(null(), message_size, message_count) };
        (!handle.is_null())
            .then_some(MessageQueue { handle })
            .ok_or(sces::value::ErrValue::InstanceCreateFailure)
    }

    fn send(&self, content: &dyn MessageContent, timeout: u32) -> RetValue<()>
    {
        unsafe { sces_mq_send(self.handle, content.as_ptr(), timeout).into() }
    }

    fn receive(&self, cache: &mut dyn MessageContent, timeout: u32) -> RetValue<()>
    {
        unsafe { sces_mq_receive(self.handle, cache.as_mut_ptr(), timeout).into() }
    }
}
