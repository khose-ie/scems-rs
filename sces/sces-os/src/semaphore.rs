/// Semaphore Trait
/// Defines the interface for semaphore operations
/// # Examples
/// ```rust
/// let semaphore = MySemaphore::new(3).unwrap();   // Create a semaphore with a maximum count of 3
/// semaphore.take();                             // Take the semaphore
/// // Critical section code here
/// semaphore.back();                             // Release the semaphore
/// ```
use sces::value::RetValue;

/// ISemaphore Trait
/// Defines the interface for semaphore operations
pub trait ISemaphore
{
    /// Create a new Semaphore instance
    /// # Arguments
    /// * `max_count: u32` - The maximum count for the semaphore
    /// # Returns
    /// * `RetValue<Self>` - Result containing the new semaphore instance or an error
    fn new(max_count: u32) -> RetValue<Self>
    where
        Self: Sized;

    /// Take the semaphore
    fn take(&self);

    /// Release the semaphore
    fn back(&self);

    /// Attempt to take the semaphore with error handling
    /// # Arguments
    /// * `timeout: u32` - The timeout duration in milliseconds
    /// # Returns
    /// * `RetValue<()>` - Result indicating success or failure
    /// # Errors
    /// * `ErrValue` - If the semaphore could not be taken within the timeout period
    fn attempt_take(&self, timeout: u32) -> RetValue<()>;
}
