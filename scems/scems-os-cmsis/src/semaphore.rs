use core::ptr::null;

use scems::value::{ErrValue, RetValue};
use scems_os::semaphore::ISemaphore;

use crate::native::*;

pub struct Semaphore
{
    handle: osSemaphoreId_t,
}

impl Semaphore
{
    pub fn new(max_count: u32) -> RetValue<Self>
    {
        let handle = unsafe { osSemaphoreNew(max_count, 0, null()) };
        if handle.is_null()
        {
            return Err(ErrValue::InstanceCreate);
        }
        Ok(Semaphore { handle })
    }
}

impl Drop for Semaphore
{
    fn drop(&mut self)
    {
        unsafe { osSemaphoreDelete(self.handle) };
    }
}

impl ISemaphore for Semaphore
{
    fn take(&self)
    {
        unsafe { osSemaphoreAcquire(self.handle, osWaitForever) };
    }

    fn back(&self)
    {
        unsafe { osSemaphoreRelease(self.handle) };
    }

    fn attempt_take(&self, timeout: u32) -> RetValue<()>
    {
        unsafe { osSemaphoreAcquire(self.handle, timeout).into() }
    }
}
