use core::cell::RefCell;

use scems::value::RetValue;

/// The trait to specific the standard method of an OS mutex.
/// Should be implemented by the real OS interfaces.
pub trait IMutex
{
    fn lock(&self);
    fn unlock(&self);
    fn attempt_lock(&self, time: u32) -> RetValue<()>;
}

/// The packed sample class with a mutex implementation.
/// To provide some functions to auto lock/unlock to access the data of the class.
/// Also get the reference of the internal sample in some no-mutex scenrio like interrupts.
pub struct MutexSample<M, S>
where
    M: Sized + IMutex,
    S: Sized,
{
    mutex: M,
    sample: RefCell<S>,
}

impl<M: IMutex, S> MutexSample<M, S>
{
    pub const fn new(mutex: M, sample: S) -> Self
    {
        Self { mutex, sample: RefCell::new(sample) }
    }

    pub fn lock_then(&self, f: impl FnOnce(&mut S)) -> RetValue<()>
    {
        self.mutex.attempt_lock(1000)?;
        f(&mut *self.sample.try_borrow_mut()?);
        self.mutex.unlock();
        Ok(())
    }

    pub fn lock_then_with<T>(&self, f: impl FnOnce(&mut S) -> RetValue<T>) -> RetValue<T>
    {
        self.mutex.attempt_lock(1000)?;
        let value = f(&mut *self.sample.try_borrow_mut()?);
        self.mutex.unlock();

        value
    }

    pub unsafe fn no_lock_then(&self, f: impl FnOnce(&mut S)) -> RetValue<()>
    {
        f(&mut *self.sample.try_borrow_mut()?);
        Ok(())
    }

    pub unsafe fn no_lock_then_with<T>(&self, f: impl FnOnce(&mut S) -> RetValue<T>)
        -> RetValue<T>
    {
        f(&mut *self.sample.try_borrow_mut()?)
    }
}

unsafe impl<M, S> Send for MutexSample<M, S> where M: IMutex {}

unsafe impl<M, S> Sync for MutexSample<M, S> where M: IMutex {}
