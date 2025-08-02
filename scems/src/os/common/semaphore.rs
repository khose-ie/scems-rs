use crate::common::result::RetValue;

pub trait ISemaphore
{
    fn take(&self);
    fn back(&self);
    fn attempt_take(&self, timeout: u32) -> RetValue<()>;
}
