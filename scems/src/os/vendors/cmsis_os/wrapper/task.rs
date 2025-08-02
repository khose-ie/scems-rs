use core::ffi::c_void;
use core::ffi::CStr;
use core::ops::Deref;
use core::ops::DerefMut;
use core::ptr::null;

use crate::common::cast::CastOpt;
use crate::common::result::ErrValue;
use crate::common::result::RetValue;
use crate::os::common::task::ITaskSample;
use crate::os::common::task::{ITask, TaskMain, TaskPriorities};
use crate::os::vendors::cmsis_os::cmsis::*;

pub struct Task
{
    handle: osThreadId_t,
    task_main_agent: TaskMainAgent,
}

impl Task
{
    pub const fn new() -> Self
    {
        Task { handle: null(), task_main_agent: TaskMainAgent::new() }
    }

    #[allow(static_mut_refs)]
    pub unsafe fn main(argument: *mut c_void)
    {
        if let Some(task_main) = (*(argument as *mut TaskMainAgent)).task_main
        {
            (*task_main).main();
        }
    }
}

impl Drop for Task
{
    fn drop(&mut self)
    {
        if let Some(x) = self.handle.cast_opt()
        {
            unsafe { osThreadTerminate(x) };
        }
    }
}

impl ITask for Task
{
    fn activate(&mut self, name: &str, stack_size: u32, pritories: TaskPriorities, main: &dyn TaskMain) -> RetValue<()>
    {
        let thread_attr = osThreadAttr_t::new(name, stack_size, pritories);

        if let None = self.handle.cast_opt()
        {
            self.task_main_agent.task_main = Some(main as *const dyn TaskMain as *mut dyn TaskMain);
            self.handle = unsafe { osThreadNew(Task::main, self.task_main_agent.as_void_ptr(), &thread_attr) };
            self.handle.cast_opt().ok_or(ErrValue::InstanceCreate)?;
        }

        Ok(())
    }

    fn deactivate(&mut self)
    {
        if let Some(_) = self.handle.cast_opt()
        {
            unsafe { osThreadTerminate(self.handle) };
            self.handle = null();
        }
    }

    fn name(&self) -> &str
    {
        unsafe { CStr::from_ptr(osThreadGetName(self.handle)).to_str().unwrap_or_default() }
    }

    fn suspand(&self) -> RetValue<()>
    {
        unsafe { osThreadSuspend(self.handle).into() }
    }

    fn resume(&self) -> RetValue<()>
    {
        unsafe { osThreadResume(self.handle).into() }
    }
}

pub struct TaskSample<T>
where
    T: TaskMain,
{
    task: Task,
    sample: T,
}

impl<T> TaskSample<T>
where
    T: TaskMain,
{
    pub const fn new(sample: T) -> Self
    {
        Self { task: Task::new(), sample }
    }
}

impl<T> Deref for TaskSample<T>
where
    T: TaskMain,
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        &self.sample
    }
}

impl<T> DerefMut for TaskSample<T>
where
    T: TaskMain,
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.sample
    }
}

impl<T> AsRef<T> for TaskSample<T>
where
    T: TaskMain,
{
    fn as_ref(&self) -> &T
    {
        &self.sample
    }
}

impl<T> AsMut<T> for TaskSample<T>
where
    T: TaskMain,
{
    fn as_mut(&mut self) -> &mut T
    {
        &mut self.sample
    }
}

impl<T> ITaskSample<T> for TaskSample<T>
where
    T: TaskMain,
{
    fn activate(&mut self, name: &str, stack_size: u32, priorities: TaskPriorities) -> RetValue<()>
    {
        self.task.activate(name, stack_size, priorities, &self.sample)
    }

    fn deactivate(&mut self)
    {
        self.task.deactivate();
    }
}

struct TaskMainAgent
{
    task_main: Option<*mut dyn TaskMain>,
}

impl TaskMainAgent
{
    pub const fn new() -> Self
    {
        TaskMainAgent { task_main: None }
    }

    pub fn as_void_ptr(&self) -> *mut c_void
    {
        self as *const TaskMainAgent as *mut c_void
    }
}
