use crate::NativeConsoleCommandsParser;

pub trait NativeConsoleCommandsExecute
{
    fn name(&self) -> &str;
    fn execute_commands(&self, commands: &mut NativeConsoleCommandsParser);
}
