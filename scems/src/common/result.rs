pub type IResult<T> = core::result::Result<T, IError>;

#[derive(Debug)]
pub enum IError
{
    None = 0,

    /// Transport a wrong parameters to the C function.
    Param = 1,

    /// The periphery is still busy and can't do the request operation.
    BusBusy = 2,

    /// Some operation over the specfic waitting time.
    Overtime = 3,

    StackOverflow = 4,

    Permission = 5,

    NullPointer = 6,

    MemAlloc = 7,

    InstanceCreate = 8,

    NotFound = 9,

    NotInclude = 10,

    NotAvailable = 11,

    /// Unknown reason errors.
    Unknown = 12,
}
