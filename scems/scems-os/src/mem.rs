use alloc::vec::Vec;
use scems::value::{ErrValue, RetValue};

pub trait SafeVec<T>
{
    fn attempt_new() -> RetValue<Self>
    where
        Self: Sized;

    fn attempt_push(&mut self, value: T) -> RetValue<()>;
}

impl<T> SafeVec<T> for Vec<T>
{
    fn attempt_new() -> RetValue<Self>
    {
        let mut vec = Vec::new();
        vec.try_reserve(4).or(Err(ErrValue::StackOverflow))?;
        Ok(vec)
    }

    fn attempt_push(&mut self, value: T) -> RetValue<()>
    {
        if self.try_reserve(1).is_ok()
        {
            self.push(value);
            Ok(())
        }
        else
        {
            Err(ErrValue::StackOverflow)
        }
    }
}
