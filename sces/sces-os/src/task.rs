/// Task Management Module
/// Defines interfaces and structures for task management in the RTOS
/// This module provides abstractions for creating, managing, and controlling tasks.
/// It includes traits for task main functions and task management blocks,
/// as well as a sample implementation to facilitate task handling.

use core::cell::RefCell;
use core::ops::{Deref, DerefMut};

use sces::value::RetValue;

/// Task Priorities
/// Defines various priority levels for tasks in the RTOS
/// The priorities range from None to RealTime, allowing for flexible task scheduling.
#[repr(C)]
pub enum TaskPriority
{
    /// Task has no priority
    None,

    /// Idle priority
    Idle,

    /// Base priority
    /// Lowest priority level for active tasks
    /// Use this priority for tasks that should run only when no other tasks are ready
    Base,

    /// Low priority
    /// Below Normal priority
    /// Use this priority for background tasks that do not require immediate attention
    Low,

    /// Normal priority
    /// Default priority level for standard tasks
    /// Tasks with this priority are scheduled fairly among other Normal priority tasks
    /// Use this priority for most application tasks
    Normal,

    /// High priority
    /// Above Normal priority but below Privilege
    /// Use this priority for tasks that require more immediate attention than Normal tasks
    /// Be cautious when using High priority to avoid starving lower priority tasks
    High,

    /// Privilege priority
    /// Higher than High priority but lower than RealTime
    /// Use this priority for tasks that require elevated importance without being time-critical
    Privilege,

    /// Real-time priority
    /// Highest priority level for time-critical tasks
    /// Tasks with this priority should be handled with care to avoid starvation of lower priority tasks
    /// Use this priority level only when absolutely necessary
    /// Tasks with RealTime priority preempt all other tasks
    RealTime,
}

/// Task Main Interface
/// Implement this trait to define the main function of a task
pub trait ITaskMain
{
    /// The entrypoint function of the task
    /// This function will be executed when the task is activated
    /// Implement the task's behavior within this method
    fn main(&mut self);
}

/// Task Interface
/// Implement this trait to define task management blocks
pub trait ITask
{
    /// Create a new task management block
    /// Returns a RetValue containing the new task instance
    fn new() -> RetValue<Self>
    where
        Self: Sized;

    /// Terminate the task
    /// This function should be called to cleanly exit the task
    /// It performs necessary cleanup operations
    fn exit();

    /// Perform a context switch to the next ready task
    /// This function yields the CPU to allow other tasks to run
    /// It is typically called in cooperative multitasking scenarios
    fn switch_to_next();

    /// Create the task instance and activate it
    /// # Arguments
    /// * `name: &str` - The name of the task
    /// * `stack: u32` - The stack size for the task
    /// * `priority: TaskPriority` - The priority level for the task
    /// * `main: &dyn ITaskMain` - The main function implementation for the task
    /// # Returns
    /// * `RetValue<()>` - Result indicating success or failure
    fn active(
        &mut self, name: &str, stack: u32, priority: TaskPriority, main: &dyn ITaskMain,
    ) -> RetValue<()>;

    /// Get the task priority
    /// Returns the current priority level of the task
    fn priority(&self) -> TaskPriority;

    /// Set task priorities
    /// # Arguments
    /// * `priority: TaskPriority` - The new priority level to be set for the task
    /// # Returns
    /// * `RetValue<&mut Self>` - Result containing a mutable reference to the task instance or an error
    fn set_priority(&mut self, priority: TaskPriority) -> RetValue<&mut Self>;

    /// Get the task name
    /// Returns the name of the task
    fn name(&self) -> &str;

    /// Suspend the task
    /// Returns a RetValue indicating success or failure
    fn suspend(&self) -> RetValue<()>;

    /// Resume the task
    /// Returns a RetValue indicating success or failure
    fn resume(&self) -> RetValue<()>;
}

/// Task Sample
/// A helper class to manage task instances and their main functions
/// T: Task implementation
/// S: Task main implementation
///
/// Example:
/// ```rust
/// struct MyTaskMain;
/// impl ITaskMain for MyTaskMain {
///    fn main(&mut self) {
///     // Task main function implementation
///   }
/// }
/// let task_sample = TaskSample::<MyTask, MyTaskMain>::new(MyTaskMain {}).unwrap();
/// task_sample.active("MyTask", 1024, TaskPriority::Normal).unwrap();
/// ```
///
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
    /// Create a new TaskSample instance
    /// # Arguments
    /// * `sample: S` - The task main implementation
    /// # Returns
    /// * `RetValue<Self>` - Result containing the new TaskSample instance or an error
    pub fn new(sample: S) -> RetValue<Self>
    {
        Ok(Self { task: RefCell::new(T::new()?), sample })
    }

    /// Activate the task with the given parameters
    /// # Arguments
    /// * `name: &str` - The name of the task
    /// * `stack: u32` - The stack size for the task
    /// * `priorities: TaskPriority` - The priority level for the task
    /// # Returns
    /// * `RetValue<&Self>` - Result containing a reference to the TaskSample instance or an error
    pub fn active(&self, name: &str, stack: u32, priorities: TaskPriority) -> RetValue<&Self>
    {
        #[allow(unused_must_use)]
        self.task.try_borrow_mut().map(|mut x| x.active(name, stack, priorities, &self.sample))?;
        Ok(self)
    }

    /// Deactivate (suspend) the task
    /// # Returns
    /// * `RetValue<&Self>` - Result containing a reference to the TaskSample instance or an error
    pub fn deactive(&self) -> RetValue<&Self>
    {
        #[allow(unused_must_use)]
        self.task.try_borrow_mut().map(|x| x.suspend())?;
        Ok(self)
    }

    /// Reactivate (resume) the task
    /// # Returns
    /// * `RetValue<&Self>` - Result containing a reference to the TaskSample instance
    pub fn reactive(&self) -> RetValue<&Self>
    {
        #[allow(unused_must_use)]
        self.task.try_borrow_mut().map(|x| x.resume())?;
        Ok(self)
    }
}

impl<T: ITask, S: ITaskMain> Deref for TaskSample<T, S>
{
    type Target = S;

    /// Get a reference to the task main implementation
    fn deref(&self) -> &Self::Target
    {
        &self.sample
    }
}

impl<T: ITask, S: ITaskMain> DerefMut for TaskSample<T, S>
{
    /// Get a mutable reference to the task main implementation
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.sample
    }
}

impl<T: ITask, S: ITaskMain> AsRef<S> for TaskSample<T, S>
{
    /// Get a reference to the task main implementation
    fn as_ref(&self) -> &S
    {
        &self.sample
    }
}

impl<T: ITask, S: ITaskMain> AsMut<S> for TaskSample<T, S>
{
    /// Get a mutable reference to the task main implementation
    fn as_mut(&mut self) -> &mut S
    {
        &mut self.sample
    }
}
