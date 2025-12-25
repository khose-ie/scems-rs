use alloc::vec::Vec;

use crate::value::{ErrValue, RetValue};

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
