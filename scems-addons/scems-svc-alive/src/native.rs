mod queue;
mod status;

use core::marker::PhantomData;

use scems::value::RetValue;
use scems_mcu::wd::WatchDogDevice;
use scems_os::mutex::MutexSample;
use scems_os::task::ITaskMain;
use scems_os::OS;

use crate::alive::{AliveWatch, AliveWatchHandle};
use crate::native::queue::AliveWatchQueue;

pub struct NativeAliveWatch<'a, O>
where
    O: OS,
{
    device: WatchDogDevice,
    cycle_time: u32,
    watch_queue: MutexSample<O::Mutex, AliveWatchQueue<'a>>,
    _marker: PhantomData<O>,
}

impl<'a, O> NativeAliveWatch<'a, O>
where
    O: OS,
{
    pub fn new(device: WatchDogDevice, mutex: O::Mutex, cycle_time: u32) -> RetValue<Self>
    {
        Ok(Self {
            device,
            cycle_time,
            watch_queue: MutexSample::new(mutex, AliveWatchQueue::new()?),
            _marker: PhantomData,
        })
    }
}

unsafe impl<'a, O> Send for NativeAliveWatch<'a, O> where O: OS {}

unsafe impl<'a, O> Sync for NativeAliveWatch<'a, O> where O: OS {}

impl<'a, O> AliveWatch for NativeAliveWatch<'a, O>
where
    O: OS,
{
    fn watch(&self, name: &'static str) -> RetValue<AliveWatchHandle>
    {
        Ok(AliveWatchHandle::new(self.watch_queue.lock_then_with(|x| x.attempt_push::<O>(name))?))
    }

    fn watch_back(&self, handle: AliveWatchHandle) -> RetValue<()>
    {
        self.watch_queue.lock_then(|x| x[*handle].set_enable(true))
    }

    fn stop_watch(&self, handle: AliveWatchHandle) -> RetValue<()>
    {
        self.watch_queue.lock_then(|x| x[*handle].set_enable(false))
    }

    fn update_alive_state(&self, handle: AliveWatchHandle)
    {
        #[allow(unused_must_use)]
        unsafe {
            self.watch_queue.no_lock_then(|x| x[*handle].update_tick(O::systick()))
        };
    }
}

impl<'a, O> ITaskMain for NativeAliveWatch<'a, O>
where
    O: OS,
{
    fn main(&mut self)
    {
        #[allow(unused_must_use)]
        self.watch_queue.lock_then(|x| x.update_all_ticks(O::systick()));
        self.device.refresh();

        loop
        {
            O::delay(self.cycle_time);

            #[allow(unused_must_use)]
            self.watch_queue
                .lock_then_with(|x| x.check_alive_time(O::systick(), self.cycle_time))
                .map(|()| self.device.refresh());
        }
    }
}
