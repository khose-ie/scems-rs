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

use core::{marker::PhantomData, ptr::NonNull};

use scems::value::{ErrValue, RetValue};

use crate::device::Handle;

pub struct SampleQueue<T, U, const N: usize>
where
    T: Handle<U> + Copy,
{
    samples: [Option<T>; N],
    _marker: PhantomData<U>,
}

impl<T, U, const N: usize> SampleQueue<T, U, N>
where
    T: Handle<U> + Copy,
{
    pub const fn new() -> Self
    {
        SampleQueue { samples: [None; N], _marker: PhantomData }
    }

    pub fn allocate(&mut self, sample: &T) -> RetValue<&mut T>
    {
        self.allocate_channel(sample, u32::MIN)
    }

    pub fn allocate_channel(&mut self, sample: &T, channel: u32) -> RetValue<&mut T>
    {
        for sample_one in self.samples.iter_mut()
        {
            if let Some(data) = sample_one
            {
                if !data.handle_value().is_null()
                    && data.handle_value().eq(&sample.handle_value())
                    && data.channel_value().eq(&channel)
                {
                    return Ok(data);
                }
            }
            else
            {
                *sample_one = Some(*sample);
                return sample_one.as_mut().ok_or(ErrValue::Unknown);
            }
        }

        Err(ErrValue::StackOverflow)
    }

    pub fn clean(&mut self, sample_handle: NonNull<U>)
    {
        self.clean_channel(sample_handle, u32::MIN);
    }

    pub fn clean_channel(&mut self, sample_handle: NonNull<U>, channel: u32)
    {
        let mut found = false;

        for idx in 0..(self.samples.len() - 1)
        {
            if let Some(data) = self.samples[idx].as_ref()
            {
                if !data.handle_value().is_null()
                    && data.handle_value().eq(&sample_handle.as_ptr())
                    && data.channel_value().eq(&channel)
                {
                    found = true;
                }

                if found.eq(&true)
                {
                    if idx < (self.samples.len() - 1)
                    {
                        self.samples[idx] = self.samples[idx + 1];
                    }
                    else
                    {
                        self.samples[idx] = None;
                    }
                }
            }
            else
            {
                break;
            }
        }
    }

    pub fn search(&mut self, sample_handle: NonNull<U>) -> RetValue<&T>
    {
        self.search_channel(sample_handle, u32::MIN)
    }

    pub fn search_channel(&mut self, sample_handle: NonNull<U>, channel: u32) -> RetValue<&T>
    {
        for sample in self.samples.iter()
        {
            if let Some(data) = sample
            {
                if !data.handle_value().is_null()
                    && data.handle_value().eq(&sample_handle.as_ptr())
                    && data.channel_value().eq(&channel)
                {
                    return Ok(data);
                }
            }
            else
            {
                break;
            }
        }

        Err(ErrValue::InstanceNotFound)
    }

    pub fn search_mut(&mut self, sample_handle: NonNull<U>) -> RetValue<&mut T>
    {
        self.search_mut_channel(sample_handle, u32::MIN)
    }

    pub fn search_mut_channel(
        &mut self, sample_handle: NonNull<U>, channel: u32,
    ) -> RetValue<&mut T>
    {
        for sample in self.samples.iter_mut()
        {
            if let Some(data) = sample
            {
                if !data.handle_value().is_null()
                    && data.handle_value().eq(&sample_handle.as_ptr())
                    && data.channel_value().eq(&channel)
                {
                    return Ok(data);
                }
            }
            else
            {
                break;
            }
        }

        Err(ErrValue::InstanceNotFound)
    }
}
