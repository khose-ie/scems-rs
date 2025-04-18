use core::cell::RefCell;
use core::ptr::null;

use crate::common::cast::CastOpt;
use crate::common::result::IError;
use crate::common::result::IResult;
use crate::os::common::mutex::IMutex;
use crate::os::common::mutex::IMutexBlock;
use crate::os::vendors::cmsis_os::cmsis::*;

pub struct Mutex
{
    handle: osMutexId_t,
}

impl Mutex
{
    pub fn new() -> IResult<Self>
    {
        let handle = unsafe { osMutexNew(null()).cast_opt().ok_or(IError::InstanceCreate) }?;
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

    fn attempt_lock(&self, time: u32) -> IResult<()>
    {
        unsafe { osMutexAcquire(self.handle, time).into() }
    }
}

pub struct MutexBlock<T>
{
    mutex: Mutex,
    value: RefCell<T>,
}

impl<T> MutexBlock<T>
{
    pub fn new(value: T) -> IResult<Self>
    {
        Ok(MutexBlock { mutex: Mutex::new()?, value: RefCell::new(value) })
    }
}

impl<T> IMutexBlock<T> for MutexBlock<T>
{
    fn lock(&self, f: impl FnOnce(&mut T))
    {
        self.mutex.lock();
        let mut value = self.value.borrow_mut();
        f(&mut value);
        self.mutex.unlock();
    }

    fn lock_with<R>(&self, f: impl FnOnce(&mut T) -> IResult<R>) -> IResult<R>
    {
        self.mutex.lock();
        let mut value = self.value.borrow_mut();
        let result = f(&mut value);
        self.mutex.unlock();
        result
    }

    fn attempt_lock_with<R>(&self, time: u32, f: impl FnOnce(&mut T) -> IResult<R>) -> IResult<R>
    {
        self.mutex.attempt_lock(time)?;
        let mut value = self.value.borrow_mut();
        let result = f(&mut value);
        self.mutex.unlock();
        result
    }
}
