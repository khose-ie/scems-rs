use core::ffi::c_void;
use core::ptr::null;

use crate::common::cast::CastOpt;
use crate::common::result::{ErrValue, RetValue};
use crate::os::common::message_queue::{IMessageQueue, MessageContent};
use crate::os::vendors::cmsis_os::cmsis::*;

pub struct MessageQueue
{
    handle: osMessageQueueId_t,
}

impl MessageQueue
{
    pub fn new(message_count: u32, message_size: u32) -> RetValue<Self>
    {
        let handle =
            unsafe { osMessageQueueNew(message_count, message_size, null()).cast_opt().ok_or(ErrValue::InstanceCreate) }?;
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
    fn launch(&self, message_content: &dyn MessageContent, timeout: u32) -> RetValue<()>
    {
        unsafe { osMessageQueuePut(self.handle, message_content.as_ptr() as *mut c_void, 0, timeout).into() }
    }

    fn receive(&self, message_content: &mut dyn MessageContent, timeout: u32) -> RetValue<()>
    {
        let mut prio: u8 = 0;
        unsafe { osMessageQueueGet(self.handle, message_content.as_ptr() as *mut c_void, &mut prio, timeout).into() }
    }
}
