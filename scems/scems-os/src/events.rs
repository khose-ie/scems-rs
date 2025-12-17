/// SCEMS OS Events Module
/// This module defines the IEvents trait for event handling mechanisms
/// in the SCEMS RTOS. It provides abstractions for creating, launching,
/// and receiving events.
/// The IEvents trait can be implemented for different RTOS backends
/// to provide a consistent API for event management across various platforms.
use scems::value::RetValue;

/// Events Interface
/// Implement this trait to define event handling mechanisms
/// for your RTOS backend.
pub trait IEvents
{
    /// Create a new Events instance
    /// # Returns
    /// * `RetValue<Self>` - Result containing the new events instance or an error
    fn new() -> RetValue<Self>
    where
        Self: Sized;

    /// Launch events
    /// # Arguments
    /// * `events: u32` - The events to be launched
    /// # Returns
    /// * `RetValue<()>` - Result indicating success or failure
    fn launch(&self, events: u32) -> RetValue<()>;

    /// Receive events
    /// # Arguments
    /// * `events: u32` - The events to be received
    /// * `timeout: u32` - The timeout duration in milliseconds
    /// # Returns
    /// * `RetValue<u32>` - Result containing the received events or an error
    fn receive(&self, events: u32, timeout: u32) -> RetValue<u32>;
}
