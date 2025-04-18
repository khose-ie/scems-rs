use crate::common::result::IResult;

pub trait IEvents
{
    fn launch(&self, events: u32) -> IResult<()>;
    fn receive(&self, events: u32, timeout: u32) -> IResult<u32>;
}
