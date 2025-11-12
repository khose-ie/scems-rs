use scems::value::RetValue;

pub trait ISemaphore
{
    fn new(max_count: u32) -> RetValue<Self> where Self: Sized;
    fn take(&self);
    fn back(&self);
    fn attempt_take(&self, timeout: u32) -> RetValue<()>;
}
