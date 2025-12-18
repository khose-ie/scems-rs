//! The common return value type and error kinds list.

use core::cell::BorrowMutError;

/// `ErrValue` includes all kinds of errors' code in sces.
///
/// Every function in sces which will return a result may has exception situation,
/// and this exception situation, will return an `ErrValue`.
/// The coller should get the code, judge it and do the related handle process.
#[derive(Debug)]
pub enum ErrValue
{
    /// No error occurred. This value always has none actual meaning because when no error,
    /// functions usually return the real value which is mark as `T` in `Result<T, E>`.
    None = 0,

    /// Transport a wrong parameters to the called function.
    Param = 1,

    /// The bus is still busy and can't do the request operation.
    Busy = 2,

    /// Some operations over the specfic waitting time, or official max time.
    Overtime = 3,

    /// The stack that you want to something into has reached its max limit count.
    StackOverflow = 4,

    /// The permission of the caller could not do the operation.
    Permission = 5,

    /// There is a null pointer or object reference in the processing.
    NullReference = 6,

    /// Memory allocation failed during the operation.
    MemAllocFailure = 7,

    /// Create some sub instance failed during the operation.
    InstanceCreateFailure = 8,

    /// The target instance could not be found during the operation.
    InstanceNotFound = 9,

    /// Attempt to crate an unique instance more than once.
    InstanceDuplicate = 10,

    /// The instance is in use and could not borrow it.
    InstanceInUse = 11,

    /// The feature includes this operation is not enabled in this distribution.
    NotInclude = 12,

    /// Some must modules of this operation are not available.
    NotAvailable = 13,

    /// Get an error when try to format a string for a series bytes.
    FormatFaliure = 14,

    /// Unknown reason errors.
    Unknown = 15,
}

impl From<BorrowMutError> for ErrValue
{
    /// When occur the `BorrowMutError`, it could be covert to `ErrValue::InstanceInUse`.
    fn from(_: BorrowMutError) -> Self
    {
        ErrValue::InstanceInUse
    }
}

/// `RetValue` is common type of return value for sces functions.
///
/// It is a packed type of RUST `Result<T, E>` type, but specificed the E with ErrValue,
/// which is also the common error code definition of sces.
pub type RetValue<T> = core::result::Result<T, ErrValue>;
