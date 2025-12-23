mod queue;
mod status;

use core::marker::PhantomData;

use log::error;
use sces::value::RetValue;
use sces_mcu::wd::WatchDogDevice;
use sces_os::mutex::MutexSample;
use sces_os::task::ITaskMain;
use sces_os::RTOS;

use crate::alive::{AliveWatch, AliveWatchHandle};
use crate::native::queue::AliveWatchQueue;
use crate::svc::AWS;

pub struct NativeAliveWatch<'a, OS>
where
    OS: RTOS,
{
    device: WatchDogDevice,
    cycle_time: u32,
    watch_queue: MutexSample<OS, AliveWatchQueue<'a>>,
    _marker: PhantomData<OS>,
}

impl<'a, OS> NativeAliveWatch<'a, OS>
where
    OS: RTOS,
{
    pub fn new(device: WatchDogDevice, cycle_time: u32) -> RetValue<Self>
    {
        Ok(Self {
            device,
            cycle_time,
            watch_queue: MutexSample::new(AliveWatchQueue::new()?)?,
            _marker: PhantomData,
        })
    }
}

unsafe impl<'a, OS> Send for NativeAliveWatch<'a, OS> where OS: RTOS {}

unsafe impl<'a, OS> Sync for NativeAliveWatch<'a, OS> where OS: RTOS {}

impl<'a, OS> AliveWatch for NativeAliveWatch<'a, OS>
where
    OS: RTOS,
{
    fn watch(&self, name: &'static str) -> RetValue<AliveWatchHandle>
    {
        Ok(AliveWatchHandle::new(
            self.watch_queue.attempt_lock_then(|x| x.attempt_push::<OS>(name))?,
        ))
    }

    fn watch_back(&self, handle: AliveWatchHandle) -> RetValue<()>
    {
        self.watch_queue.attempt_lock()?[handle].set_enable(true);
        Ok(())
    }

    fn stop_watch(&self, handle: AliveWatchHandle) -> RetValue<()>
    {
        self.watch_queue.attempt_lock()?[handle].set_enable(false);
        Ok(())
    }

    fn update_alive_state(&self, handle: AliveWatchHandle)
    {
        self.watch_queue.lock()[handle].update_tick(OS::ticks());
    }
}

impl<'a, OS> ITaskMain for NativeAliveWatch<'a, OS>
where
    OS: RTOS,
{
    fn main(&mut self)
    {
        #[allow(unused_must_use)]
        self.watch_queue.lock().update_all_ticks(OS::ticks());
        self.device.refresh();

        loop
        {
            OS::delay(self.cycle_time);

            #[allow(unused_must_use)]
            self.watch_queue
                .attempt_lock_then(|x| x.check_alive_time(OS::ticks(), self.cycle_time))
                .inspect(|()| self.device.refresh())
                .inspect_err(|_| error!("{AWS} Some tasks near death, don't refresh Watch Dog."));
        }
    }
}
