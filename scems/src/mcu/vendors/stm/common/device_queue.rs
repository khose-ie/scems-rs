//! A queue data structure to save the pointer of peripheral devices.
//!
//! The main function of DeviceQueue is to save the pointer of peripheral device which has been set the event agent.
//! The reason why we need such a data structure is came from the status of STM32 HAL libraries.
//! In STM32 HAL libraries, all same-kind peripheral will have the same callback function, and the handle pointer 
//! will be transported as the argument of the callback function, the caller should judgement the argument, and do 
//! the divided action with some branch logic like `if-else` statement.
//!
//! In scems, the event agent, an interface who provide the specification of the callback function, will be registered
//! to every peripheral. So we need to map the single callback function with the multiple event agent, by the only 
//! information of pointer of handle.
//!
//! A naturally methods is to create a map table, save the device instance and handle pointer, when the device be 
//! registered event agent, this device will be add to the map table. And the single callback function will find 
//! the device according to the argument of handle pointer.
//! So, that is what the DeviceQueue is.

use core::marker::PhantomData;
use core::ptr::null_mut;

use crate::common::cast::CastOpt;
use crate::common::result::{IError, IResult};
use crate::mcu::common::HandlePtr;

/// A queue template to be used to map the peripheral device and the handler pointer of it.
/// 
/// To instantiate this template, you need provide 3 types or information.
/// 
/// 1. T, the type of the handle of the peripheral device.
///    Usually, we use handle with pointer, but the T is not a pointer type, you should provide the data type of it, 
/// 2. U, the type of the peripheral device.
/// 3. N, the count or the max length of the queue. It always be decided by the count of the peripheral device of the 
///    MCU you choice.
pub struct DeviceQueue<T, U, const N: usize>
where
    U: HandlePtr<T>,
{
    samples: [Option<*mut U>; N],
    _marker: PhantomData<T>,
}

impl<T, U, const N: usize> DeviceQueue<T, U, N>
where
    U: HandlePtr<T>,
{
    pub const fn new() -> Self
    {
        DeviceQueue { samples: [None; N], _marker: PhantomData }
    }

    #[rustfmt::skip]
    pub fn alloc(&mut self, sample_value: *mut U) -> IResult<()>
    {
        let sample_value = sample_value.cast_opt().ok_or(IError::Param)?;
        let mut sample_temp: *mut U = null_mut();

        for sample in self.samples.iter()
        {
            if let Some(data) = sample
            {
                if !data.is_null() && unsafe { (**data).handle_ptr() == (*sample_value).handle_ptr() }
                {
                    sample_temp = *data;
                    break;
                }
            }
        }

        let None = sample_temp.cast_opt() else { return Ok(()); };
        sample_temp = null_mut();

        for sample in self.samples.iter_mut()
        {
            if *sample == None
            {
                *sample = Some(sample_value);
                sample_temp = sample_value;
                break;
            }
        }

        let _ = sample_temp.cast_opt().ok_or(IError::StackOverflow)?;
        Ok(())
    }

    #[rustfmt::skip]
    pub fn clean(&mut self, sample_value: *mut U)
    {
        let Some(sample_value) = sample_value.cast_opt() else { return; };

        for sample in self.samples.iter_mut()
        {
            if let Some(data) = sample
            {
                if !data.is_null() && *data == sample_value
                {
                    *sample = None;
                    break;
                }
            }
        }
    }

    #[rustfmt::skip]
    pub fn find(&self, handle_value: *mut T) -> IResult<*mut U>
    {
        let Some(handle_value) = handle_value.cast_opt() else { return Err(IError::Param); };
        let mut sample_value: *mut U = null_mut();

        for sample in self.samples.iter()
        {
            if let Some(data) = sample
            {
                if !data.is_null() && unsafe { (**data).handle_ptr() == handle_value }
                {
                    sample_value = *data;
                    break;
                }
            }
        }

        let Some(x) = sample_value.cast_opt() else { return Err(IError::NotFound); };
        Ok(x)
    }
}
