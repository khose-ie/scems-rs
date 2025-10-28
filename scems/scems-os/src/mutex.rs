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

    pub fn lock<T>(&self, f: impl FnOnce(&mut S) -> RetValue<T>) -> RetValue<T>
    {
        self.mutex.lock();
        let value = f(&mut self.sample.borrow_mut());
        self.mutex.unlock();

        value
    }

    pub fn attempt_lock<T>(&self, time: u32, f: impl FnOnce(&mut S) -> RetValue<T>) -> RetValue<T>
    {
        self.mutex.attempt_lock(time)?;
        let value = f(&mut self.sample.borrow_mut());
        self.mutex.unlock();

        value
    }

    pub fn samples(&self) -> &S
    {
        unsafe { &*self.sample.as_ptr() }
    }

    pub fn samples_mut(&mut self) -> &mut S
    {
        self.sample.get_mut()
    }
}
