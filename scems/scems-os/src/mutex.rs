use scems::value::RetValue;

pub trait IMutex
{
    fn lock(&self);
    fn unlock(&self);
    fn attempt_lock(&self, time: u32) -> RetValue<()>;
}

pub trait IMutexSample: IMutex
{
    fn with_lock<V>(&mut self, f: impl FnOnce(&mut Self) -> RetValue<V>) -> RetValue<V>
    {
        self.lock();
        let value = f(self);
        self.unlock();

        value
    }

    fn attempt_with_lock<R>(
        &mut self, time: u32, f: impl FnOnce(&mut Self) -> RetValue<R>,
    ) -> RetValue<R>
    {
        self.attempt_lock(time)?;
        let value = f(self);
        self.unlock();

        value
    }
}
