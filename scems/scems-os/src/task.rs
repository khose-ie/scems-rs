use core::ops::{Deref, DerefMut};

use scems::value::RetValue;

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

pub trait ITaskMain
{
    fn main(&mut self);
}

pub trait ITask
{
    fn activate(
        &mut self, name: &str, stack_size: u32, pritories: TaskPriorities, main: &dyn ITaskMain,
    ) -> RetValue<()>;
    fn deactivate(&mut self);
    fn name(&self) -> &str;
    fn suspand(&self) -> RetValue<()>;
    fn resume(&self) -> RetValue<()>;
}

pub struct TaskSample<T, S>
where
    T: Sized + ITask,
    S: Sized + ITaskMain,
{
    task: T,
    sample: S,
}

impl<T: ITask, S: ITaskMain> TaskSample<T, S>
{
    pub const fn new(task: T, sample: S) -> Self
    {
        Self { task, sample }
    }

    pub fn initialize(
        &mut self, name: &str, stack_size: u32, priority: TaskPriorities,
    ) -> RetValue<()>
    {
        self.task.activate(name, stack_size, priority, &self.sample)
    }

    pub fn finalize(&mut self)
    {
        self.task.deactivate();
    }
}

impl<T: ITask, S: ITaskMain> Deref for TaskSample<T, S>
{
    type Target = S;

    fn deref(&self) -> &Self::Target
    {
        &self.sample
    }
}

impl<T: ITask, S: ITaskMain> DerefMut for TaskSample<T, S>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.sample
    }
}

impl<T: ITask, S: ITaskMain> AsRef<S> for TaskSample<T, S>
{
    fn as_ref(&self) -> &S
    {
        &self.sample
    }
}

impl<T: ITask, S: ITaskMain> AsMut<S> for TaskSample<T, S>
{
    fn as_mut(&mut self) -> &mut S
    {
        &mut self.sample
    }
}
