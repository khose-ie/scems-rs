use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::{AtomicU8, Ordering};

use crate::value::{ErrValue, RetValue};

pub struct StaticCell<T>
{
    state: AtomicU8,
    value: UnsafeCell<MaybeUninit<T>>,
}

impl<T> StaticCell<T>
{
    pub const fn new() -> Self
    {
        Self { state: AtomicU8::new(0), value: UnsafeCell::new(MaybeUninit::uninit()) }
    }

    pub fn set(&self, v: T) -> RetValue<&T>
    {
        self.state
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| ErrValue::InstanceDuplicate)?;

        unsafe { (*self.value.get()).write(v) };
        Ok(unsafe { (*self.value.get()).assume_init_ref() })
    }

    pub fn get(&self) -> Option<&'static T>
    {
        (self.state.load(Ordering::Acquire) != 0)
            .then_some(unsafe { (*self.value.get()).assume_init_ref() })
    }
}
