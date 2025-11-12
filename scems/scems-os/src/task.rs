use core::cell::RefCell;
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
    fn new() -> RetValue<Self>
    where
        Self: Sized;
    fn active(
        &mut self, name: &str, stack: u32, priorities: TaskPriorities, main: &dyn ITaskMain,
    ) -> RetValue<()>;
    fn set_priorities(&mut self, pritories: TaskPriorities) -> RetValue<&mut Self>;
    fn name(&self) -> &str;
    fn suspand(&self) -> RetValue<()>;
    fn resume(&self) -> RetValue<()>;
}

pub struct TaskSample<T, S>
where
    T: Sized + ITask,
    S: Sized + ITaskMain,
{
    task: RefCell<T>,
    sample: S,
}

impl<T: ITask, S: ITaskMain> TaskSample<T, S>
{
    pub fn new(sample: S) -> RetValue<Self>
    {
        Ok(Self { task: RefCell::new(T::new()?), sample })
    }

    pub fn active(&self, name: &str, stack: u32, priorities: TaskPriorities) -> RetValue<&Self>
    {
        #[allow(unused_must_use)]
        self.task.try_borrow_mut().map(|mut x| x.active(name, stack, priorities, &self.sample))?;
        Ok(self)
    }

    pub fn active_back(&self) -> RetValue<&Self>
    {
        #[allow(unused_must_use)]
        self.task.try_borrow_mut().map(|x| x.resume())?;
        Ok(self)
    }

    pub fn deactive(&self) -> RetValue<&Self>
    {
        #[allow(unused_must_use)]
        self.task.try_borrow_mut().map(|x| x.suspand())?;
        Ok(self)
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
