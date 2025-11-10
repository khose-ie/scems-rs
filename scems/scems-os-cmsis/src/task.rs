use core::ffi::{c_void, CStr};
use core::ops::Not;
use core::ptr::null;

use scems::value::{ErrValue, RetValue};
use scems_os::task::{ITask, ITaskMain, TaskPriorities};

use crate::native::*;

pub struct Task
{
    handle: osThreadId_t,
    main_agent: TaskMainAgent,
}

impl Task
{
    pub const fn new() -> Self
    {
        Task { handle: null(), main_agent: TaskMainAgent::new() }
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
        if self.handle.is_null().not()
        {
            unsafe { osThreadTerminate(self.handle) };
        }
    }
}

impl ITask for Task
{
    fn activate(
        &mut self, name: &str, stack_size: u32, pritories: TaskPriorities, main: &dyn ITaskMain,
    ) -> RetValue<()>
    {
        let thread_attr = osThreadAttr_t::new(name, stack_size, pritories);

        if self.handle.is_null()
        {
            self.main_agent.task_main = Some(main as *const dyn ITaskMain as *mut dyn ITaskMain);
            self.handle =
                unsafe { osThreadNew(Task::main, self.main_agent.as_void_ptr(), &thread_attr) };

            if self.handle.is_null()
            {
                return Err(ErrValue::InstanceCreateFailure);
            }
        }

        Ok(())
    }

    fn deactivate(&mut self)
    {
        if self.handle.is_null().not()
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

/// Create a agent class to pack the pointer of `dyn ITaskMain`.
/// Because the fat pointer `dyn ITaskMain` can be set to C function as thin pointer.
struct TaskMainAgent
{
    task_main: Option<*mut dyn ITaskMain>,
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
