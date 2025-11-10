use log::Log;
use scems::value::RetValue;

pub trait Console: Send + Sync + Log
{
    fn accept_dispatch(&self, exe: &'static dyn ConsoleExecute) -> RetValue<()>;
}

pub trait ConsoleExecute
{
    fn exe_name(&self) -> &str;
    fn exe_with_cmds(&self, cmds: &mut ConsoleCommands) -> RetValue<()>;
}

pub struct ConsoleCommands<'a>
{
    cmds: &'a [u8],
    position: usize,
}

impl<'a> ConsoleCommands<'a>
{
    pub fn new(cmds: &'a [u8]) -> Self
    {
        Self { cmds, position: 0 }
    }
}

impl<'a> Iterator for ConsoleCommands<'a>
{
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item>
    {
        let mut pos = self.position;

        while pos < self.cmds.len() && self.cmds[pos].is_ascii_whitespace()
        {
            pos += 1;
        }

        self.cmds.get(pos)?;

        let s = pos;

        while pos < self.cmds.len() && !self.cmds[pos].is_ascii_whitespace()
        {
            pos += 1;
        }

        self.position = pos;

        Some(&self.cmds[s..pos])
    }
}
