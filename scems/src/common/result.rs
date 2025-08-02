//! The common return value type and error kinds list.

/// `ErrValue` includes all kinds of errors' code in scems.
/// 
/// Every function in scems which will return a result may has exception situation,
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
    BusBusy = 2,

    /// Some operations over the specfic waitting time, or official max time.
    Overtime = 3,

    /// The stack that you want to something into has reached its max limit count.
    StackOverflow = 4,

    /// The permission of the caller could not do the operation.
    Permission = 5,

    /// There is a null pointer or object reference in the processing.
    NullReference = 6,

    /// Memory allocation failed during the operation.
    MemAlloc = 7,

    /// Create some sub instance failed during the operation.
    InstanceCreate = 8,

    /// The target instance could not be found during the operation.
    InstanceNotFound = 9,

    /// The feature includes this operation is not enabled in this distribution.
    NotInclude = 10,

    /// Some must modules of this operation are not available.
    NotAvailable = 11,

    /// Unknown reason errors.
    Unknown = 12,
}

/// `RetValue` is common type of return value for scems functions.
/// 
/// It is a packed type of RUST `Result<T, E>` type, but specificed the E with ErrValue, 
/// which is also the common error code definition of scems.
pub type RetValue<T> = core::result::Result<T, ErrValue>;
