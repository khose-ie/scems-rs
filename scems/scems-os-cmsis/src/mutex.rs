use core::ops::Not;
use core::ptr::null;

use scems::value::ErrValue;
use scems::value::RetValue;
use scems_os::mutex::IMutex;

use crate::native::*;

pub struct Mutex
{
    handle: osMutexId_t,
}

impl Mutex
{
    pub fn new() -> RetValue<Self>
    {
        let handle = unsafe { osMutexNew(null()) };
        handle.is_null().not().then_some(handle).ok_or(ErrValue::InstanceCreateFailure)?;
        Ok(Mutex { handle })
    }
}

impl Drop for Mutex
{
    fn drop(&mut self)
    {
        unsafe { osMutexDelete(self.handle) };
    }
}

impl IMutex for Mutex
{
    fn lock(&self)
    {
        unsafe { osMutexAcquire(self.handle, osWaitForever) };
    }

    fn unlock(&self)
    {
        unsafe { osMutexRelease(self.handle) };
    }

    fn attempt_lock(&self, time: u32) -> RetValue<()>
    {
        unsafe { osMutexAcquire(self.handle, time).into() }
    }
}
