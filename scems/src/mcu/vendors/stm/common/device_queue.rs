use core::marker::PhantomData;
use core::ptr::null_mut;

use crate::common::cast::CastOpt;
use crate::common::result::{IError, IResult};
use crate::mcu::common::HandlePtr;

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
