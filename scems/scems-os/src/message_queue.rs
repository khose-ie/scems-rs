use scems::value::RetValue;

pub trait IMessageQueue
{
    fn launch(&self, content: &dyn MessageContent, timeout: u32) -> RetValue<()>;
    fn receive(&self, cache: &mut dyn MessageContent, timeout: u32) -> RetValue<()>;
}

pub trait MessageContent
{
    fn as_ptr(&self) -> *const u8;
}
