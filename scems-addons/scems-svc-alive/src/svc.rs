use scems::value::{ErrValue, RetValue};

use crate::alive::AliveWatch;

static mut SVC: Option<&'static dyn AliveWatch> = None;

pub struct AliveWatchService;

impl AliveWatchService
{
    pub fn initialize<T>(instance: &'static T) -> RetValue<()>
    where
        T: AliveWatch,
    {
        #[allow(static_mut_refs)]
        unsafe { SVC.is_none().then_some(()).ok_or(ErrValue::InstanceDuplicate)? };
        unsafe { SVC = Some(instance) };
        Ok(())
    }

    pub fn instance() -> &'static dyn AliveWatch
    {
        unsafe { SVC.unwrap() }
    }
}
