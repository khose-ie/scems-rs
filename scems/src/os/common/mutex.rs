use crate::common::result::RetValue;

pub trait IMutex
{
    fn lock(&self);
    fn unlock(&self);
    fn attempt_lock(&self, time: u32) -> RetValue<()>;
}

pub trait IMutexBlock<T>
{
    fn lock(&self, f: impl FnOnce(&mut T));
    fn lock_with<R>(&self, f: impl FnOnce(&mut T) -> RetValue<R>) -> RetValue<R>;
    fn attempt_lock_with<R>(&self, time: u32, f: impl FnOnce(&mut T) -> RetValue<R>) -> RetValue<R>;
}
