use log::LevelFilter;
use scems::value::{ErrValue, RetValue};

use crate::Console;

pub const CS: &str = "<ConsoleService>";

static mut SVC: Option<&'static dyn Console> = None;

pub struct ConsoleService;

impl ConsoleService
{
    pub fn initialize<T>(instance: &'static T, level: LevelFilter) -> RetValue<()>
    where
        T: Console,
    {
        log::set_logger(instance).or(Err(ErrValue::InstanceDuplicate))?;
        log::set_max_level(level);

        unsafe { SVC = Some(instance) };
        Ok(())
    }

    pub fn instance() -> &'static dyn Console
    {
        unsafe { SVC.unwrap() }
    }
}
