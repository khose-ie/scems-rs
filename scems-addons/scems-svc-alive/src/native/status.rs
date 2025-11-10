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

    pub fn is_alive(&self, tick: u32, max_time: u32) -> bool
    {
        tick - self.alive_tick > max_time
    }
}
