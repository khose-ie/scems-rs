use core::ffi::{c_void, CStr};
use core::ptr::null;

use sces::value::{ErrValue, RetValue};
use sces_os::task::{ITask, ITaskMain, TaskPriority};

use crate::native::*;

pub struct Task
{
    handle: osThreadId_t,
    main_agent: TaskMainAgent,
}

impl Task
{
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
        (!self.handle.is_null()).then_some(self.handle).map(|x| unsafe { osThreadTerminate(x) });
    }
}

impl ITask for Task
{
    fn new() -> RetValue<Self>
    where
        Self: Sized,
    {
        Ok(Task { handle: null(), main_agent: TaskMainAgent::new() })
    }

    fn exit()
    {
        unsafe { osThreadExit() };
    }

    fn switch_to_next()
    {
        unsafe { osThreadYield() };
    }

    fn active(
        &mut self, name: &str, stack: u32, pritories: TaskPriority, main: &dyn ITaskMain,
    ) -> RetValue<()>
    {
        let attr = osThreadAttr_t::new(name, stack, pritories);

        if self.handle.is_null()
        {
            self.main_agent.set_main(main);
            self.handle = unsafe { osThreadNew(Task::main, self.main_agent.as_void_ptr(), &attr) };
        }

        (!self.handle.is_null()).then_some(()).ok_or(ErrValue::InstanceCreateFailure)
    }

    fn priority(&self) -> TaskPriority
    {
        todo!()
    }

    fn set_priority(&mut self, pritories: TaskPriority) -> RetValue<&mut Self>
    {
        RetValue::from(unsafe { osThreadSetPriority(self.handle, pritories.into()).into() })?;
        Ok(self)
    }

    fn name(&self) -> &str
    {
        unsafe { CStr::from_ptr(osThreadGetName(self.handle)).to_str().unwrap_or_default() }
    }

    fn suspend(&self) -> RetValue<()>
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

    pub fn set_main(&mut self, main: &dyn ITaskMain)
    {
        self.task_main = Some(main as *const dyn ITaskMain as *mut dyn ITaskMain);
    }

    pub fn as_void_ptr(&self) -> *mut c_void
    {
        self as *const TaskMainAgent as *mut c_void
    }
}
