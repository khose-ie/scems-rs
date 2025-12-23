use core::{ffi::CStr, ptr::{null_mut}};

use sces::value::{ErrValue, RetValue};
use sces::os::task::{ITask, ITaskMain, TaskMainAgent, TaskPriority, TaskState};

use crate::os::native::*;

pub struct Task
{
    handle: ScesTaskHandle,
    main_agent: TaskMainAgent,
}

impl Task
{
    pub fn from(handle: ScesTaskHandle) -> Self
    {
        Task { handle, main_agent: TaskMainAgent::new() }
    }
}

impl Drop for Task
{
    fn drop(&mut self)
    {
        self.main_agent.inspect(|_| unsafe { sces_task_delete(self.handle) });
    }
}

impl ITask for Task
{
    fn new() -> RetValue<Self>
    where
        Self: Sized,
    {
        Ok(Task { handle: null_mut(), main_agent: TaskMainAgent::new() })
    }

    #[rustfmt::skip]
    fn active(
        &mut self, name: &str, stack: u32, priority: TaskPriority, main: &dyn ITaskMain,
    ) -> RetValue<()>
    {
        self.handle.is_null().then_some(()).ok_or(ErrValue::InstanceDuplicate)?;

        self.main_agent.set_main(main);
        self.handle = 
            unsafe { sces_task_create(name.as_ptr(), Task::main, self.main_agent.as_ptr(), stack, priority.into()) };
    
        (!self.handle.is_null()).then_some(()).ok_or(ErrValue::InstanceCreateFailure)
    }

    fn name(&self) -> &str
    {
        unsafe {  CStr::from_ptr(sces_task_name(self.handle) as *const i8).to_str().unwrap_or("") } 
    }

    fn stack_size(&self) -> u32
    {
        unsafe { sces_task_stack_size(self.handle) }
    }

    fn priority(&self) -> TaskPriority
    {
        unsafe { sces_task_priority(self.handle).into() }
    }

    fn state(&self) -> TaskState
    {
        unsafe { sces_task_state(self.handle).into() }
    }

    fn set_priority(&mut self, priority: TaskPriority) -> RetValue<()>
    {
        unsafe { sces_task_set_priority(self.handle, priority.into()).into() }
    }

    fn suspend(&self) -> RetValue<()>
    {
        unsafe { sces_task_suspend(self.handle).into() }
    }

    fn resume(&self) -> RetValue<()>
    {
        unsafe { sces_task_resume(self.handle).into() }
    }
}
