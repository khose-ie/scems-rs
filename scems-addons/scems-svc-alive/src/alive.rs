use core::ops::Deref;

use scems::value::RetValue;

pub struct AliveWatchHandle
{
    num: usize,
}

impl AliveWatchHandle
{
    pub(crate) const fn new(num: usize) -> Self
    {
        Self { num }
    }
}

impl Deref for AliveWatchHandle
{
    type Target = usize;

    fn deref(&self) -> &Self::Target
    {
        &self.num
    }
}

pub trait AliveWatch: Send + Sync
{
    fn watch(&self, name: &'static str) -> RetValue<AliveWatchHandle>;
    fn watch_back(&self, handle: AliveWatchHandle) -> RetValue<()>;
    fn stop_watch(&self, handle: AliveWatchHandle) -> RetValue<()>;
    fn update_alive_state(&self, handle: AliveWatchHandle);
}
