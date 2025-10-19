use scems::value::RetValue;

pub trait IMessageQueue
{
    fn launch(&self, message_content: &dyn MessageContent, timeout: u32) -> RetValue<()>;
    fn receive(&self, message_content: &mut dyn MessageContent, timeout: u32) -> RetValue<()>;
}

pub trait MessageContent
{
    fn as_ptr(&self) -> *const u8;
}
