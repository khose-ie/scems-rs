pub type WatchDogDevice = &'static mut dyn WatchDogCtrl;

pub trait WatchDogCtrl
{
    fn refresh(&self);
}
