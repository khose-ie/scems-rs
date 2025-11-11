use log::error;
use scems::value::{ErrValue, RetValue};

use crate::svc::AWS;

pub struct AliveStatus<'a>
{
    name: &'a str,
    enable: bool,
    alive_tick: u32,
}

impl<'a> AliveStatus<'a>
{
    pub fn new(name: &'a str, alive_tick: u32) -> Self
    {
        Self { name, enable: true, alive_tick }
    }

    pub fn name(&self) -> &'a str
    {
        self.name
    }

    pub fn set_enable(&mut self, enable: bool)
    {
        self.enable = enable;
    }

    pub fn update_tick(&mut self, tick: u32)
    {
        self.alive_tick = tick;
    }

    pub fn check_alive(&self, tick: u32, max_time: u32) -> RetValue<()>
    {
        let past = tick - self.alive_tick;

        (past > max_time)
            .then_some(())
            .ok_or(ErrValue::Overtime)
            .inspect_err(|_| error!("{AWS} {} is near death over {}", self.name, past))
    }
}
