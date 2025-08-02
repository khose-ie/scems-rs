use scems::common::result::{ErrValue, RetValue};
use scems::os::common::mem::IMemQueue;
use scems::os::vendors::mem::MemQueue;

use crate::service::console::{ConsoleCommandDispatches, ConsoleCommandExecution};

pub struct ConsoleServiceDispatches<'a>
{
    execution_queue: MemQueue<&'a dyn ConsoleCommandExecution, 8>,
}

impl<'a> ConsoleServiceDispatches<'a>
{
    pub fn new() -> RetValue<Self>
    {
        Ok(Self { execution_queue: MemQueue::new()? })
    }

    #[inline]
    pub fn assign_command_execution(&mut self, execution: &'a dyn ConsoleCommandExecution) -> RetValue<usize>
    {
        self.execution_queue.push(&execution)
    }

    #[inline]
    pub fn remove_command_execution(&mut self, execution: &'a dyn ConsoleCommandExecution)
    {
        self.execution_queue.remove(&execution);
    }
}

impl<'a> ConsoleCommandDispatches for ConsoleServiceDispatches<'a>
{
    fn dispatch_and_execute(&self, command: &[u8], response: &mut [u8]) -> RetValue<()>
    {
        const MAX_ARGS: usize = 16;

        let mut args: [&[u8]; MAX_ARGS] = [&[]; MAX_ARGS];

        let mut idx = 0;
        let mut count = 0;

        while idx < command.len() && count < MAX_ARGS
        {
            while idx < command.len() && (command[idx] == b' ' || command[idx] == b'\r' || command[idx] == b'\n')
            {
                idx += 1;
            }

            if idx >= command.len()
            {
                break;
            }

            let start = idx;

            while idx < command.len() && command[idx] != b' ' && command[idx] != b'\r' && command[idx] != b'\n'
            {
                idx += 1;
            }

            args[count] = &command[start..idx];
            count += 1;
        }

        if count == 0
        {
            return Err(ErrValue::Param);
        }

        let cmd = args[0];
        let params = &args[1..count];

        let mut execution: Option<&dyn ConsoleCommandExecution> = None;

        for exec in self.execution_queue.iter()
        {
            if exec.console_command_name().as_bytes().eq(cmd)
            {
                execution = Some(*exec);
                break;
            }
        }

        let execution = execution.ok_or(ErrValue::NotFound)?;
        Ok(execution.execute_console_command(params, response)?)
    }
}
