/// Share Mutex Trait
/// Defines the interface for share mutex operations
/// # Examples
/// ```rust
/// let sxmutex = MySxMutex::new();
/// sxmutex.involve();   // Involve the share mutex
/// // Critical section code here
/// sxmutex.leave();     // Leave the share mutex
/// ```

/// Safety: The implementer must ensure that the share mutex operations
/// are safe to be called from multiple threads concurrently.
pub trait ISxMutex
{
    /// 
    fn involve(&mut self);

    ///
    fn leave(&mut self);

    ///
    fn keep(&mut self);
    fn release(&mut self);
}
