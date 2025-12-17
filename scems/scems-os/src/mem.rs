/// Memory Zone Interface
/// Implement this trait to define memory zones
/// for your RTOS backend.

use core::usize;

use alloc::vec::Vec;
use scems::value::{ErrValue, RetValue};

/// The trait to specific the standard method of an OS memory zone.
/// Should be implemented by the real OS interfaces.
pub trait IMemZone
{
    /// Get the block size of the memory zone in bytes
    /// Returns the size of each block in bytes
    /// # Returns
    /// * `u32` - Size of each block in bytes
    fn block_size(&self) -> u32;

    /// Get the total number of blocks in the memory zone
    /// Returns the total number of blocks available in the memory zone
    /// # Returns
    /// * `u32` - Total number of blocks
    fn block_count(&self) -> u32;

    /// Get the starting address of the memory zone
    /// Returns a slice representing the memory zone
    /// # Returns
    /// * `&'static [u8]` - Slice of the memory zone
    fn address(&self) -> &'static [u8];
}

/// Memory Zone Implementation
/// A generic memory zone implementation with fixed block size and count.
/// SN: Size of each block in bytes
/// CN: Number of blocks in the memory zone
pub struct MemZone<const SN: usize, const CN: usize>
{
    block_size: u32,
    block_count: u32,
    address: [[u8; SN]; CN],
}

impl<const SN: usize, const CN: usize> MemZone<SN, CN>
{
    /// Create a new MemZone instance
    pub const fn new() -> Self
    {
        Self { block_size: SN as u32, block_count: CN as u32, address: [[0; SN]; CN] }
    }
}

impl<const SN: usize, const CN: usize> IMemZone for MemZone<SN, CN>
{
    /// Get the block size of the memory zone in bytes
    /// Returns the size of each block in bytes
    /// # Returns
    /// * `u32` - Size of each block in bytes
    fn block_size(&self) -> u32
    {
        self.block_size
    }

    /// Get the total number of blocks in the memory zone
    /// Returns the total number of blocks available in the memory zone
    /// # Returns
    /// * `u32` - Total number of blocks
    fn block_count(&self) -> u32
    {
        self.block_count
    }

    /// Get the starting address of the memory zone
    /// Returns a slice representing the memory zone
    /// # Returns
    /// * `&'static [u8]` - Slice of the memory zone
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

/// Safe Vector Extension Trait
/// Provides methods to create and manipulate vectors with error handling
/// to prevent stack overflow.
pub trait SafeVec<T>
{
    /// Create a new vector with initial capacity
    /// # Returns
    /// * `RetValue<Self>` - Result containing the new vector or an error
    /// if memory allocation fails
    /// # Errors
    /// * `ErrValue::StackOverflow` - If memory allocation fails while creating the vector
    fn attempt_new() -> RetValue<Self>
    where
        Self: Sized;

    /// Attempt to push a value into the vector
    /// # Arguments
    /// * `value: T` - The value to be pushed into the vector
    /// # Returns
    /// * `RetValue<()>` - Result indicating success or failure
    /// # Errors
    /// * `ErrValue::StackOverflow` - If memory allocation fails while pushing the value
    fn attempt_push(&mut self, value: T) -> RetValue<()>;
}

/// Implement SafeVec for Vec<T>
/// Provides safe methods to create and manipulate vectors
/// with error handling to prevent stack overflow.
impl<T> SafeVec<T> for Vec<T>
{
    /// Create a new vector with initial capacity
    /// # Returns
    /// * `RetValue<Self>` - Result containing the new vector or an error
    /// if memory allocation fails
    /// # Errors
    /// * `ErrValue::StackOverflow` - If memory allocation fails while creating the vector
    fn attempt_new() -> RetValue<Self>
    {
        let mut vec = Vec::new();
        vec.try_reserve(4).or(Err(ErrValue::StackOverflow))?;
        Ok(vec)
    }

    /// Attempt to push a value into the vector
    /// # Arguments
    /// * `value: T` - The value to be pushed into the vector
    /// # Returns
    /// * `RetValue<()>` - Result indicating success or failure
    /// # Errors
    /// * `ErrValue::StackOverflow` - If memory allocation fails while pushing the value
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
