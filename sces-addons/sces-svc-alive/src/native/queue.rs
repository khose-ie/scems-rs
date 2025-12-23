use core::ops::{Index, IndexMut};

use alloc::vec::Vec;
use sces::value::{ErrValue, RetValue};
use sces_os::mem::SafeVec;
use sces_os::RTOS;

use crate::native::status::AliveStatus;
use crate::AliveWatchHandle;

pub struct AliveWatchQueue<'a>
{
    queue: Vec<AliveStatus<'a>>,
}

impl<'a> AliveWatchQueue<'a>
{
    pub fn new() -> RetValue<Self>
    {
        Ok(Self { queue: Vec::attempt_new()? })
    }

    pub fn attempt_push<OS>(&mut self, name: &'a str) -> RetValue<usize>
    where
        OS: RTOS,
    {
        (!self.queue.iter().any(|x| x.name() == name))
            .then_some(())
            .ok_or(ErrValue::InstanceDuplicate)?;

        self.queue.attempt_push(AliveStatus::new(name, OS::ticks()))?;
        Ok(self.queue.len() - 1)
    }

    pub fn update_all_ticks(&mut self, tick: u32)
    {
        self.queue.iter_mut().for_each(|x| x.update_tick(tick));
    }

    pub fn check_alive_time(&self, now: u32, max_time: u32) -> RetValue<()>
    {
        self.queue.iter().try_for_each(|x| x.check_alive(now, max_time))
    }
}

impl<'a> Index<AliveWatchHandle> for AliveWatchQueue<'a>
{
    type Output = AliveStatus<'a>;

    fn index(&self, index: AliveWatchHandle) -> &Self::Output
    {
        &self.queue[*index]
    }
}

impl<'a> IndexMut<AliveWatchHandle> for AliveWatchQueue<'a>
{
    fn index_mut(&mut self, index: AliveWatchHandle) -> &mut Self::Output
    {
        &mut self.queue[*index]
    }
}
