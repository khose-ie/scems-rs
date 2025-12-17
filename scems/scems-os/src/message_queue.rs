/// IMessageQueue Trait and MessageContent Trait
/// Defines the interface for message queue operations
/// and the content structure for messages.
/// # Examples
/// ```rust
/// struct MyMessageContent {
///     data: [u8; 128],
/// }
/// impl MessageContent for MyMessageContent {
///     fn as_ptr(&self) -> *const u8 {
///         self.data.as_ptr()
///     }
/// }
/// let message_queue = MyMessageQueue::new(10, 128).unwrap();
/// let message = MyMessageContent { data: [0; 128] };
/// message_queue.launch(&message, 1000).unwrap();
/// ```
use scems::value::RetValue;

/// IMessageQueue Trait
/// Defines the interface for message queue operations
/// and the content structure for messages.
pub trait IMessageQueue
{
    /// Create a new Message Queue instance
    /// # Arguments
    /// * `message_count: u32` - The maximum number of messages in the queue
    /// * `message_size: u32` - The size of each message in bytes
    /// # Returns
    /// * `RetValue<Self>` - Result containing the new message queue instance or an error
    fn new(message_count: u32, message_size: u32) -> RetValue<Self>
    where
        Self: Sized;

    /// Launch a message into the queue
    /// # Arguments
    /// * `content: &dyn MessageContent` - The message content to be sent
    /// * `timeout: u32` - The timeout duration in milliseconds
    /// # Returns
    /// * `RetValue<()>` - Result indicating success or failure
    fn launch(&self, content: &dyn MessageContent, timeout: u32) -> RetValue<()>;

    /// Receive a message from the queue
    /// # Arguments
    /// * `cache: &mut dyn MessageContent` - The buffer to store the received message
    /// * `timeout: u32` - The timeout duration in milliseconds
    /// # Returns
    /// * `RetValue<()>` - Result indicating success or failure
    fn receive(&self, cache: &mut dyn MessageContent, timeout: u32) -> RetValue<()>;
}

/// MessageContent Trait
/// Defines the structure for message content
/// that can be sent and received through the message queue.
pub trait MessageContent
{
    /// Get a pointer to the message content data
    /// # Returns
    /// * `*const u8` - Pointer to the message content data
    fn as_ptr(&self) -> *const u8;
}
