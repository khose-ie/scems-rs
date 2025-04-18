use core::ops::{Deref, DerefMut};

use crate::common::result::IResult;

pub trait ITask
{
    fn activate(&mut self, name: &str, stack_size: u32, pritories: TaskPriorities, main: &dyn TaskMain) -> IResult<()>;
    fn deactivate(&mut self);
    fn name(&self) -> &str;
    fn suspand(&self) -> IResult<()>;
    fn resume(&self) -> IResult<()>;
}

pub trait ITaskSample<T>
where
    Self: Deref + DerefMut + AsRef<T> + AsMut<T>,
{
    fn activate(&mut self, name: &str, stack_size: u32, priorities: TaskPriorities) -> IResult<()>;
    fn deactivate(&mut self);
}

pub trait TaskMain
{
    fn main(&mut self);
}

#[repr(C)]
pub enum TaskPriorities
{
    None,
    Idle,
    Base,
    Low,
    Normal,
    High,
    Privilege,
    RealTime,
}
