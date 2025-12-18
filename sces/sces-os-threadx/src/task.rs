use core::{ffi::c_char, ptr::null_mut};

use sces::value::RetValue;
use sces_os::task::{ITask, ITaskMain, TaskPriority};

use crate::native::*;

pub struct Task
{
    handle: *mut TX_THREAD,
    // main_agent: TaskMainAgent,
}

impl ITask for Task
{
    fn new() -> RetValue<Self>
    where
        Self: Sized,
    {
        Ok(Self { handle: null_mut() })
    }

    fn active(
        &mut self, name: &str, stack: u32, priorities: TaskPriority, main: &dyn ITaskMain,
    ) -> RetValue<()>
    {
        unsafe {
            _txe_thread_create(
                self.handle,
                name.as_ptr() as *const c_char,
                Some(main.main),
                0,
                null_mut(),
                stack,
                priorities.into(),
                priorities.into(),
                0,
                1,
                0,
            )
            .into()
        }
    }

    fn set_priorities(&mut self, pritories: TaskPriority) -> RetValue<&mut Self>
    {
        todo!()
    }

    fn name(&self) -> &str
    {
        todo!()
    }

    fn suspand(&self) -> RetValue<()>
    {
        todo!()
    }

    fn resume(&self) -> RetValue<()>
    {
        todo!()
    }
}
