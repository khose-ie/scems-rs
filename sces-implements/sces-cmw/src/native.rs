use sces::value::{ErrValue, RetValue};

/// SCES return value enumeration
///
/// Represents standard return values for SCES C functions
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScesRetVal
{
    Ok = 0,
    ErrParam = 1,
    ErrBusy = 2,
    ErrTimeout = 3,
    ErrStackOverflow = 4,
    ErrPermission = 5,
    ErrNullRef = 6,
    ErrMemAllocFailure = 7,
    ErrLowLevelFailure = 8,
    ErrInstanceCreateFailure = 9,
    ErrInstanceNotFound = 10,
    ErrInstanceDuplicate = 11,
    ErrInstanceInUse = 12,
    ErrInstanceInvalid = 13,
    ErrNotSupport = 14,
    ErrNotAvailable = 15,
    ErrFormatFailure = 16,
    ErrUnknown = 255,
}

impl From<ScesRetVal> for ErrValue
{
    fn from(ret: ScesRetVal) -> Self
    {
        match ret
        {
            ScesRetVal::Ok => ErrValue::None,
            ScesRetVal::ErrParam => ErrValue::Param,
            ScesRetVal::ErrBusy => ErrValue::Busy,
            ScesRetVal::ErrTimeout => ErrValue::Timeout,
            ScesRetVal::ErrStackOverflow => ErrValue::StackOverflow,
            ScesRetVal::ErrPermission => ErrValue::Permission,
            ScesRetVal::ErrNullRef => ErrValue::NullReference,
            ScesRetVal::ErrMemAllocFailure => ErrValue::MemAllocFailure,
            ScesRetVal::ErrLowLevelFailure => ErrValue::LowLevelFailure,
            ScesRetVal::ErrInstanceCreateFailure => ErrValue::InstanceCreateFailure,
            ScesRetVal::ErrInstanceNotFound => ErrValue::InstanceNotFound,
            ScesRetVal::ErrInstanceDuplicate => ErrValue::InstanceDuplicate,
            ScesRetVal::ErrInstanceInUse => ErrValue::InstanceInUse,
            ScesRetVal::ErrInstanceInvalid => ErrValue::InstanceInvalid,
            ScesRetVal::ErrNotSupport => ErrValue::NotSupport,
            ScesRetVal::ErrNotAvailable => ErrValue::NotAvailable,
            ScesRetVal::ErrFormatFailure => ErrValue::FormatFailure,
            ScesRetVal::ErrUnknown => ErrValue::Unknown,
        }
    }
}

impl From<ErrValue> for ScesRetVal
{
    fn from(err: ErrValue) -> Self
    {
        match err
        {
            ErrValue::None => ScesRetVal::Ok,
            ErrValue::Param => ScesRetVal::ErrParam,
            ErrValue::Busy => ScesRetVal::ErrBusy,
            ErrValue::Timeout => ScesRetVal::ErrTimeout,
            ErrValue::StackOverflow => ScesRetVal::ErrStackOverflow,
            ErrValue::Permission => ScesRetVal::ErrPermission,
            ErrValue::NullReference => ScesRetVal::ErrNullRef,
            ErrValue::MemAllocFailure => ScesRetVal::ErrMemAllocFailure,
            ErrValue::LowLevelFailure => ScesRetVal::ErrLowLevelFailure,
            ErrValue::InstanceCreateFailure => ScesRetVal::ErrInstanceCreateFailure,
            ErrValue::InstanceNotFound => ScesRetVal::ErrInstanceNotFound,
            ErrValue::InstanceDuplicate => ScesRetVal::ErrInstanceDuplicate,
            ErrValue::InstanceInUse => ScesRetVal::ErrInstanceInUse,
            ErrValue::InstanceInvalid => ScesRetVal::ErrInstanceInvalid,
            ErrValue::NotSupport => ScesRetVal::ErrNotSupport,
            ErrValue::NotAvailable => ScesRetVal::ErrNotAvailable,
            ErrValue::FormatFailure => ScesRetVal::ErrFormatFailure,
            ErrValue::Unknown => ScesRetVal::ErrUnknown,
        }
    }
}

impl ScesRetVal
{
    pub fn map<T>(&self, value: T) -> RetValue<T>
    {
        match self
        {
            ScesRetVal::Ok => Ok(value),
            _ => Err((*self).into()),
        }
    }
}
