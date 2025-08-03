use super::EventLaunch;
use crate::common::result::RetValue;

pub type CanDevice = &'static mut dyn CanCtrl;

pub trait CanCtrl
where
    Self: EventLaunch<dyn CanDeviceEventAgent>,
{
    fn activate(&self) -> RetValue<()>;
    fn deactivate(&self) -> RetValue<()>;
    fn transmit(&self, can_message: &CanMessage, timeout: u32) -> RetValue<()>;
    fn receive(&self, can_message: &mut CanMessage, timeout: u32) -> RetValue<()>;
    fn async_transmit(&self, can_message: &CanMessage) -> RetValue<()>;
    fn async_receive(&mut self, can_message: &mut CanMessage);
}

pub trait CanDeviceEventAgent
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
