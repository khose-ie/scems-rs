use core::usize;

use alloc::vec::Vec;
use scems::value::{ErrValue, RetValue};

pub trait IMemZone
{
    fn block_size(&self) -> u32;

    fn block_count(&self) -> u32;

    fn address(&self) -> &'static [u8];
}

pub struct MemZone<const SN: usize, const CN: usize>
{
    block_size: u32,
    block_count: u32,
    address: [[u8; SN]; CN],
}

impl<const SN: usize, const CN: usize> MemZone<SN, CN>
{
    pub const fn new() -> Self
    {
        Self { block_size: SN as u32, block_count: CN as u32, address: [[0; SN]; CN] }
    }
}

impl<const SN: usize, const CN: usize> IMemZone for MemZone<SN, CN>
{
    fn block_size(&self) -> u32
    {
        self.block_size
    }

    fn block_count(&self) -> u32
    {
        self.block_count
    }

    fn address(&self) -> &'static [u8]
    {
        unsafe {
            core::slice::from_raw_parts(
                self.address.as_ptr() as *const u8,
                (self.block_size as usize) * (self.block_count as usize),
            )
        }
    }
}

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
