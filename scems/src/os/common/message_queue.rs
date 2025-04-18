use crate::common::result::IResult;

pub trait IMessageQueue
{
    fn launch(&self, message_content: &dyn MessageContent, timeout: u32) -> IResult<()>;
    fn receive(&self, message_content: &mut dyn MessageContent, timeout: u32)-> IResult<()>;
}

pub trait MessageContent
{
    fn as_ptr(&self) -> *const u8;
}
