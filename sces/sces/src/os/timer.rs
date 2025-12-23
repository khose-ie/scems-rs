use core::ffi::c_void;

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
use crate::value::RetValue;

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

pub enum TimerState
{
    /// The timer is currently idle
    /// Indicates that the timer is not running
    Idle,

    /// The timer is currently active
    /// Indicates that the timer is running and counting down
    Active,

    /// The timer has expired
    /// Indicates that the timer has reached its set duration and triggered its event
    Expired,

    /// The timer has been deleted
    /// Indicates that the timer instance has been removed and is no longer valid
    Deleted,

    /// An error occurred with the timer
    /// Indicates that an unexpected issue has arisen during timer operations
    Error,

    /// The timer state is unknown
    Unknown,
}

/// ITimer Trait
/// Defines the interface for timer operations
pub trait ITimer
{
    /// Timer expiration callback
    /// This function is called when the timer reaches its set duration
    extern "C" fn on_time_over(argument: *mut c_void)
    {
        if let Some(agent) = unsafe { TimerEventAgent::from(argument).as_mut() }
        {
            if let Some(event) = agent.event()
            {
                event.on_time_over();
            }
        }
    }

    /// Create a new Timer instance
    /// # Arguments
    /// * `mode: TimerMode` - The mode of timer (Once or Periodic)
    /// # Returns
    /// * `RetValue<Self>` - Result containing the new timer instance or an error
    fn new(mode: TimerMode) -> RetValue<Self>
    where
        Self: Sized;

    /// Get the mode of the timer
    /// # Returns
    /// * `TimerMode` - The mode of the timer (Once or Periodic)
    fn mode(&self) -> TimerMode;

    /// Get the current state of the timer
    /// # Returns
    /// * `TimerState` - The current state of the timer
    fn state(&self) -> TimerState;

    /// Activate the timer
    /// # Arguments
    /// * `times: u32` - The duration for the timer in milliseconds
    /// * `event: &dyn ITimerEvent` - The event handler for timer expiration
    /// # Returns
    /// * `RetValue<()>` - Result indicating success or failure
    fn active(&mut self, times: u32, event: &dyn ITimerEvent) -> RetValue<()>;

    /// Terminate the timer
    fn terminate(&mut self);
}

/// ITimerEvent Trait
/// Defines the event handling for timer expiration
pub trait ITimerEvent
{
    /// Handle timer expiration event
    /// Called when the timer reaches its set duration
    fn on_time_over(&self) {}
}

pub struct TimerEventAgent
{
    event: Option<*mut dyn ITimerEvent>,
}

impl TimerEventAgent
{
    pub fn new() -> Self
    {
        Self { event: None }
    }

    pub fn from(event: *mut c_void) -> *mut Self
    {
        event as *mut Self
    }

    pub fn event(&self) -> Option<&dyn ITimerEvent>
    {
        unsafe { self.event.map(|e| &*e) }
    }

    pub fn set_event(&mut self, event: &dyn ITimerEvent)
    {
        self.event = Some(event as *const dyn ITimerEvent as *mut dyn ITimerEvent);
    }

    pub fn as_ptr(&self) -> *mut c_void
    {
        self as *const Self as *mut c_void
    }
}
