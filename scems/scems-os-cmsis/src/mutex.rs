use core::cell::RefCell;
use core::ptr::null;

use scems::value::ErrValue;
use scems::value::RetValue;
use scems_os::mutex::IMutex;
use scems_os::mutex::IMutexBlock;

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

        if handle.is_null()
        {
            return Err(ErrValue::InstanceCreate);
        }

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

pub struct MutexBlock<T>
{
    mutex: Mutex,
    value: RefCell<T>,
}

impl<T> MutexBlock<T>
{
    pub fn new(value: T) -> RetValue<Self>
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

    fn lock_with<R>(&self, f: impl FnOnce(&mut T) -> RetValue<R>) -> RetValue<R>
    {
        self.mutex.lock();
        let mut value = self.value.borrow_mut();
        let result = f(&mut value);
        self.mutex.unlock();
        result
    }

    fn attempt_lock_with<R>(&self, time: u32, f: impl FnOnce(&mut T) -> RetValue<R>) -> RetValue<R>
    {
        self.mutex.attempt_lock(time)?;
        let mut value = self.value.borrow_mut();
        let result = f(&mut value);
        self.mutex.unlock();
        result
    }
}
