use core::{
    cell::{RefCell, RefMut},
    ops::{Deref, DerefMut},
};

use scems::value::RetValue;

use crate::RTOS;

/// The trait to specific the standard method of an OS mutex.
/// Should be implemented by the real OS interfaces.
pub trait IMutex
{
    fn new() -> RetValue<Self>
    where
        Self: Sized;
    fn lock(&self);
    fn unlock(&self);
    fn attempt_lock(&self, time: u32) -> RetValue<()>;
}

pub struct MutexGuid<'a, S>
{
    mutex: &'a dyn IMutex,
    sample: RefMut<'a, S>,
}

impl<'a, S> MutexGuid<'a, S>
{
    pub fn new(mutex: &'a dyn IMutex, sample: RefMut<'a, S>) -> Self
    {
        Self { mutex, sample }
    }
}

impl<'a, S> Drop for MutexGuid<'a, S>
{
    fn drop(&mut self)
    {
        self.mutex.unlock();
    }
}

impl<'a, S> Deref for MutexGuid<'a, S>
{
    type Target = S;

    fn deref(&self) -> &Self::Target
    {
        &self.sample
    }
}

impl<'a, S> DerefMut for MutexGuid<'a, S>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.sample
    }
}

/// The packed sample class with a mutex implementation.
/// To provide some functions to auto lock/unlock to access the data of the class.
/// Also get the reference of the internal sample in some no-mutex scenrio like interrupts.
pub struct MutexSample<OS, S>
where
    OS: Sized + RTOS,
    S: Sized,
{
    mutex: OS::Mutex,
    sample: RefCell<S>,
}

impl<OS, S> MutexSample<OS, S>
where
    OS: RTOS,
{
    pub fn new(sample: S) -> RetValue<Self>
    {
        Ok(Self { mutex: OS::Mutex::new()?, sample: RefCell::new(sample) })
    }

    pub fn lock(&self) -> MutexGuid<S>
    {
        self.mutex.lock();
        MutexGuid::new(&self.mutex, self.sample.borrow_mut())
    }

    pub fn attempt_lock(&self) -> RetValue<MutexGuid<S>>
    {
        self.mutex.attempt_lock(1000)?;
        Ok(MutexGuid::new(&self.mutex, self.sample.try_borrow_mut()?))
    }

    pub fn attempt_lock_then<T, F>(&self, f: F) -> RetValue<T>
    where
        F: FnOnce(&mut S) -> RetValue<T>,
    {
        self.mutex.attempt_lock(1000)?;
        let value = f(&mut *self.sample.try_borrow_mut()?);
        self.mutex.unlock();

        value
    }
}

unsafe impl<OS, S> Send for MutexSample<OS, S> where OS: RTOS {}

unsafe impl<OS, S> Sync for MutexSample<OS, S> where OS: RTOS {}
