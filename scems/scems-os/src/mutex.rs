use core::cell::{RefCell, RefMut};
/// The mutex module provides an abstraction for mutex operations
/// in the SCEMS RTOS. It defines the IMutex trait that outlines
/// the standard methods for mutex handling, including creation,
/// locking, unlocking, and attempting to lock with a timeout.
/// Additionally, it provides the MutexSample struct, which encapsulates
/// a sample data structure protected by a mutex, allowing for safe
/// concurrent access. The MutexGuid struct is used to manage the
/// lifetime of the locked data, ensuring that the mutex is properly
/// unlocked when the data is no longer needed.
/// This module is designed to be implemented by real OS interfaces,
/// providing a consistent API for mutex operations across different
/// RTOS backends.
/// The MutexSample struct also offers methods for locking the mutex
/// and accessing the internal sample data, as well as attempting
/// to lock the mutex with error handling.
/// The module ensures thread-safe access to shared resources
/// in a multitasking environment.
/// It leverages Rust's ownership and borrowing principles
/// to provide safe and efficient mutex handling.
/// This module is part of the SCEMS OS library, which aims to
/// provide abstractions and implementations for various RTOS functionalities.
/// It is designed to be modular and extensible, allowing for easy integration
/// with different RTOS backends.
use core::ops::{Deref, DerefMut};

use scems::value::RetValue;

use crate::RTOS;

/// Mutex Interface
/// Implement this trait to define mutex handling mechanisms
/// for your RTOS backend.
/// The trait includes methods for creating a new mutex,
/// locking, unlocking, and attempting to lock with a timeout.
/// This trait should be implemented by the real OS interfaces.
pub trait IMutex
{
    /// Create a new Mutex instance
    /// # Returns
    /// * `RetValue<Self>` - Result containing the new mutex instance or an error
    fn new() -> RetValue<Self>
    where
        Self: Sized;

    /// Lock the mutex
    fn lock(&self);

    /// Unlock the mutex
    fn unlock(&self);

    /// Attempt to lock the mutex with a timeout
    /// # Arguments
    /// * `time: u32` - The timeout duration in milliseconds
    /// # Returns
    /// * `RetValue<()>` - Result indicating success or failure
    fn attempt_lock(&self, time: u32) -> RetValue<()>;
}

/// Mutex Guided Accessor
/// This struct provides a guided access to a sample data structure
/// protected by a mutex. It implements Deref and DerefMut traits
/// to allow easy access to the internal sample data.
/// The mutex is automatically unlocked when the MutexGuid instance
/// goes out of scope, ensuring safe and proper mutex handling.
/// This struct is used in conjunction with the MutexSample struct
/// to provide safe concurrent access to shared resources.
pub struct MutexGuid<'a, S>
{
    mutex: &'a dyn IMutex,
    sample: RefMut<'a, S>,
}

impl<'a, S> MutexGuid<'a, S>
{
    /// Create a new MutexGuid instance
    /// # Arguments
    /// * `mutex: &'a dyn IMutex` - Reference to the mutex
    /// * `sample: RefMut<'a, S>` - Mutable reference to the sample data
    /// # Returns
    /// * `Self` - New MutexGuid instance
    pub fn new(mutex: &'a dyn IMutex, sample: RefMut<'a, S>) -> Self
    {
        Self { mutex, sample }
    }
}

impl<'a, S> Drop for MutexGuid<'a, S>
{
    /// Automatically unlock the mutex when the MutexGuid instance goes out of scope
    /// This ensures that the mutex is properly released
    /// and prevents deadlocks in concurrent scenarios.
    fn drop(&mut self)
    {
        self.mutex.unlock();
    }
}

impl<'a, S> Deref for MutexGuid<'a, S>
{
    type Target = S;

    /// Get a reference to the sample data
    fn deref(&self) -> &Self::Target
    {
        &self.sample
    }
}

impl<'a, S> DerefMut for MutexGuid<'a, S>
{
    /// Get a mutable reference to the sample data
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.sample
    }
}

/// Mutex Sample
/// This struct encapsulates a sample data structure
/// protected by a mutex. It provides methods to create
/// a new MutexSample instance, lock the mutex,
/// and access the internal sample data.
/// The MutexSample struct ensures safe concurrent access
/// to the sample data by leveraging the mutex locking mechanism.
/// It also provides methods to attempt to lock the mutex
/// with error handling.
/// This struct is designed to be used in multitasking environments
/// where shared resources need to be protected from concurrent access.
pub struct MutexSample<OS, S>
where
    OS: Sized + RTOS,
    S: Sized,
{
    mutex: OS::Mutex,
    sample: RefCell<S>,
}

impl<OS, S> MutexSample<OS, S>
where
    OS: RTOS,
{
    /// Create a new MutexSample instance
    /// # Arguments
    /// * `sample: S` - The sample data to be protected by the mutex
    /// # Returns
    /// * `RetValue<Self>` - Result containing the new MutexSample instance or an error
    pub fn new(sample: S) -> RetValue<Self>
    {
        Ok(Self { mutex: OS::Mutex::new()?, sample: RefCell::new(sample) })
    }

    /// Lock the mutex and get a guided access to the sample data
    /// # Returns
    /// * `MutexGuid<S>` - Guided access to the sample data with the mutex locked
    /// # Examples
    /// ```rust
    /// let mutex_sample = MutexSample::new(MySampleStruct::new()).unwrap();
    /// let mut sample = mutex_sample.lock().modify_data();
    /// ```
    pub fn lock(&self) -> MutexGuid<S>
    {
        self.mutex.lock();
        MutexGuid::new(&self.mutex, self.sample.borrow_mut())
    }

    /// Attempt to lock the mutex with error handling
    /// # Returns
    /// * `RetValue<MutexGuid<S>>` - Result containing the guided access to
    /// the sample data with the mutex locked or an error
    /// # Errors
    /// * `ErrValue` - If the mutex could not be locked within the timeout period
    /// # Examples
    /// ```rust
    /// let mutex_sample = MutexSample::new(MySampleStruct::new()).unwrap();
    /// mutex_sample.attempt_lock().map(|mut sample| { sample.modify_data() });
    /// ```
    pub fn attempt_lock(&self) -> RetValue<MutexGuid<S>>
    {
        self.mutex.attempt_lock(1000)?;
        Ok(MutexGuid::new(&self.mutex, self.sample.try_borrow_mut()?))
    }

    /// Attempt to lock the mutex and execute a closure with the sample data
    /// # Arguments
    /// * `f: F` - Closure that takes a mutable reference to the sample data
    /// and returns a RetValue<T>
    /// # Returns
    /// * `RetValue<T>` - Result containing the return value of the closure or an error
    /// # Errors
    /// * `ErrValue` - If the mutex could not be locked within the timeout period
    /// # Examples
    /// ```rust
    /// let mutex_sample = MutexSample::new(MySampleStruct::new()).unwrap();
    /// mutex_sample.attempt_lock_then(|sample| {
    ///     sample.modify_data();
    ///     Ok(())
    /// });
    /// ```
    pub fn attempt_lock_then<T, F>(&self, f: F) -> RetValue<T>
    where
        F: FnOnce(&mut S) -> RetValue<T>,
    {
        self.mutex.attempt_lock(1000)?;
        let value = f(&mut *self.sample.try_borrow_mut()?);
        self.mutex.unlock();

        value
    }
}

/// Safety: MutexSample can be sent and shared between threads  
unsafe impl<OS, S> Send for MutexSample<OS, S> where OS: RTOS {}

/// Safety: MutexSample can be shared between threads
unsafe impl<OS, S> Sync for MutexSample<OS, S> where OS: RTOS {}
