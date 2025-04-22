use crate::common::result::IResult;

use super::EventLaunch;

pub trait Can
where
    Self: EventLaunch<dyn CanEventAgent>,
{
    fn activate(&self) -> IResult<()>;
    fn deactivate(&self) -> IResult<()>;

    fn transmit(&self, can_message: &CanMessage, timeout: u32) -> IResult<()>;
    fn receive(&self, can_message: &mut CanMessage, timeout: u32) -> IResult<()>;
    fn async_transmit(&self, can_message: &CanMessage) -> IResult<()>;
    fn async_receive(&mut self, can_message: &mut CanMessage);
}

pub trait CanEvent
{
    fn on_can_message_receive(&mut self) {}
    fn on_can_error(&mut self) {}
}

pub trait CanEventAgent
{
    fn on_can_message_receive(&self) {}
    fn on_can_error(&self) {}
}

#[derive(Default)]
pub struct CanMessage
{
    pub head: CanMessageHead,
    pub data: CanMessageData,
}

#[derive(Default)]
#[allow(non_snake_case)]
pub struct CanMessageHead
{
    pub STD_ID: u32,
    pub EXT_ID: u32,
    pub IDE: u32,
    pub RTR: u32,
    pub DLC: u32,
}

#[derive(Default)]
pub struct CanMessageData
{
    pub content: [u8; 8],
}
