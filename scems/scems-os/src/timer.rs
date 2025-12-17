/// Timer trait and related definitions
/// Defines the interface for timer operations
/// along with timer modes and event handling.
/// # Examples
/// ```rust
/// struct MyTimerEvent;
/// impl TimerEvent for MyTimerEvent {
///     fn on_time_over(&mut self) {
///         // Handle timer expiration
///     }
/// }
/// let mut timer = MyTimer::new(TimerMode::Periodic, MyTimerEvent);
/// timer.start(1000).unwrap(); // Start timer for 1000 milliseconds
/// ```

use scems::value::RetValue;

/// ITimer Trait
/// Defines the interface for timer operations
pub trait ITimer
{
    /// Start the timer
    /// # Arguments
    /// * `times: u32` - The duration for the timer in milliseconds
    /// # Returns
    /// * `RetValue<()>` - Result indicating success or failure
    fn start(&mut self, times: u32) -> RetValue<()>;

    /// Stop the timer
    fn stop(&mut self);

    /// Check if the timer is active
    /// # Returns
    /// * `bool` - True if the timer is active, false otherwise
    fn actived(&self) -> bool;
}

/// TimerMode Enum
/// Defines the operating modes for the timer
#[derive(Clone, Copy)]
pub enum TimerMode
{
    /// One-shot timer mode
    /// The timer triggers once after the specified duration
    /// and then stops.
    Once,

    /// Periodic timer mode
    /// The timer triggers repeatedly at the specified duration
    /// intervals until stopped.
    Periodic,
}

/// TimerEvent Trait
/// Defines the event handling for timer expiration
pub trait TimerEvent
{
    /// Handle timer expiration event
    /// Called when the timer reaches its set duration
    fn on_time_over(&self) {}
}
