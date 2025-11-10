use core::ops::{Index, IndexMut};

use alloc::vec::Vec;
use scems::value::{ErrValue, RetValue};
use scems_os::mem::SafeVec;
use scems_os::OS;

use crate::native::status::AliveStatus;

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

    pub fn attempt_push<O>(&mut self, name: &'a str) -> RetValue<usize>
    where
        O: OS,
    {
        (!self.queue.iter().any(|x| x.name() == name))
            .then_some(())
            .ok_or(ErrValue::InstanceDuplicate)?;

        self.queue.attempt_push(AliveStatus::new(name, O::systick()))?;
        Ok(self.queue.len() - 1)
    }

    pub fn update_all_ticks(&mut self, tick: u32)
    {
        self.queue.iter_mut().for_each(|x| x.update_tick(tick));
    }

    pub fn check_alive_time(&self, now: u32, max_time: u32) -> RetValue<()>
    {
        self.queue
            .iter()
            .try_for_each(|x| x.is_alive(now, max_time).then_some(()).ok_or(ErrValue::Overtime))
    }
}

impl<'a> Index<usize> for AliveWatchQueue<'a>
{
    type Output = AliveStatus<'a>;

    fn index(&self, index: usize) -> &Self::Output
    {
        &self.queue[index]
    }
}

impl<'a> IndexMut<usize> for AliveWatchQueue<'a>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output
    {
        &mut self.queue[index]
    }
}
