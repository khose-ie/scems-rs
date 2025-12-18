use core::ops::{Deref, DerefMut};

use sces::value::RetValue;

pub trait ITask
{
    fn activate(
        &mut self, name: &str, stack_size: u32, pritories: TaskPriority, main: &dyn TaskMain,
    ) -> RetValue<()>;
    fn deactivate(&mut self);
    fn name(&self) -> &str;
    fn suspand(&self) -> RetValue<()>;
    fn resume(&self) -> RetValue<()>;
}

pub trait ITaskSample<T>
where
    Self: Deref + DerefMut + AsRef<T> + AsMut<T>,
{
    fn activate(&mut self, name: &str, stack_size: u32, priorities: TaskPriority)
        -> RetValue<()>;
    fn deactivate(&mut self);
}

pub trait TaskMain
{
    fn main(&mut self);
}

#[repr(C)]
pub enum TaskPriority
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
