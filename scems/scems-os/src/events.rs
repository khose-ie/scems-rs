use scems::value::RetValue;

pub trait IEvents
{
    fn launch(&self, events: u32) -> RetValue<()>;
    fn receive(&self, events: u32, timeout: u32) -> RetValue<u32>;
}
