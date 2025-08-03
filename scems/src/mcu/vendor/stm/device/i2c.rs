use core::mem::transmute;

use crate::common::result::{ErrValue, RetValue};
use crate::mcu::common::i2c::{I2cMasterCtrl, I2cMasterDeviceEventAgent};
use crate::mcu::common::i2c::{I2cMemCtrl, I2cMemDeviceEventAgent, I2cMemWide};
use crate::mcu::common::i2c::{I2cSlaveCtrl, I2cSlaveDeviceEventAgent};
use crate::mcu::common::EventLaunch;
pub use crate::mcu::vendor::stm::native::i2c::I2C_HandleTypeDef;
use crate::mcu::vendor::stm::native::i2c::*;
use crate::mcu::vendor::stm::sample_queue::SampleQueue;
use crate::mcu::vendor::stm::{Handle, I2C_COUNT};

/////////////////////////////////////////////////////////////////////////////
// I2C struct
/////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
enum I2c
{
    Mem(I2cMem),
    Master(I2cMaster),
    Slave(I2cSlave),
}

impl I2c
{
    fn new_mem(handle: *mut I2C_HandleTypeDef) -> RetValue<Self>
    {
        if handle.is_null()
        {
            return Err(ErrValue::Param);
        }

        Ok(I2c::Mem(I2cMem { handle, event_handle: None }))
    }

    fn new_master(handle: *mut I2C_HandleTypeDef) -> RetValue<Self>
    {
        if handle.is_null()
        {
            return Err(ErrValue::Param);
        }

        Ok(I2c::Master(I2cMaster { handle, event_handle: None }))
    }

    fn new_slave(handle: *mut I2C_HandleTypeDef) -> RetValue<Self>
    {
        if handle.is_null()
        {
            return Err(ErrValue::Param);
        }

        Ok(I2c::Slave(I2cSlave { handle, event_handle: None }))
    }
}

impl Handle<I2C_HandleTypeDef> for I2c
{
    fn handle_value(&self) -> *mut I2C_HandleTypeDef
    {
        match self
        {
            I2c::Mem(mem) => mem.handle,
            I2c::Master(master) => master.handle,
            I2c::Slave(slave) => slave.handle,
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// I2C struct for memory operation
/////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct I2cMem
{
    handle: *mut I2C_HandleTypeDef,
    event_handle: Option<*const dyn I2cMemDeviceEventAgent>,
}

impl EventLaunch<dyn I2cMemDeviceEventAgent> for I2cMem
{
    fn set_event_agent(&mut self, event_handle: &'static dyn I2cMemDeviceEventAgent)
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn I2cMemDeviceEventAgent) });
    }

    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
    }
}

impl I2cMemCtrl for I2cMem
{
    fn mem_write(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &[u8], timeout: u32) -> RetValue<()>
    {
        unsafe {
            HAL_I2C_Mem_Write(self.handle, saddr, maddr, mwide.into(), data.as_ptr(), data.len() as u16, timeout).into()
        }
    }

    fn mem_read(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &mut [u8], timeout: u32) -> RetValue<()>
    {
        unsafe {
            HAL_I2C_Mem_Read(self.handle, saddr, maddr, mwide.into(), data.as_ptr(), data.len() as u16, timeout).into()
        }
    }

    fn async_mem_write(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &[u8]) -> RetValue<()>
    {
        unsafe {
            HAL_I2C_Mem_Write_DMA(self.handle, saddr, maddr, mwide.into(), data.as_ptr(), data.len() as u16).into()
        }
    }

    fn async_mem_read(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &mut [u8]) -> RetValue<()>
    {
        unsafe {
            HAL_I2C_Mem_Read_DMA(self.handle, saddr, maddr, mwide.into(), data.as_ptr(), data.len() as u16).into()
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// I2C struct as a master
/////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct I2cMaster
{
    handle: *mut I2C_HandleTypeDef,
    event_handle: Option<*const dyn I2cMasterDeviceEventAgent>,
}

impl EventLaunch<dyn I2cMasterDeviceEventAgent> for I2cMaster
{
    fn set_event_agent(&mut self, event_handle: &'static dyn I2cMasterDeviceEventAgent)
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn I2cMasterDeviceEventAgent) });
    }

    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
    }
}

impl I2cMasterCtrl for I2cMaster
{
    fn transmit(&self, saddr: u16, data: &[u8], timeout: u32) -> RetValue<()>
    {
        unsafe { HAL_I2C_Master_Transmit(self.handle, saddr, data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn receive(&self, saddr: u16, data: &mut [u8], timeout: u32) -> RetValue<()>
    {
        unsafe { HAL_I2C_Master_Receive(self.handle, saddr, data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn async_transmit(&self, saddr: u16, data: &[u8]) -> RetValue<()>
    {
        unsafe { HAL_I2C_Master_Transmit_DMA(self.handle, saddr, data.as_ptr(), data.len() as u16).into() }
    }

    fn async_receive(&self, saddr: u16, data: &mut [u8]) -> RetValue<()>
    {
        unsafe { HAL_I2C_Master_Receive_DMA(self.handle, saddr, data.as_ptr(), data.len() as u16).into() }
    }
}

/////////////////////////////////////////////////////////////////////////////
// I2C struct as a slave
/////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct I2cSlave
{
    handle: *mut I2C_HandleTypeDef,
    event_handle: Option<*const dyn I2cSlaveDeviceEventAgent>,
}

impl EventLaunch<dyn I2cSlaveDeviceEventAgent> for I2cSlave
{
    fn set_event_agent(&mut self, event_handle: &'static dyn I2cSlaveDeviceEventAgent)
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn I2cSlaveDeviceEventAgent) });
    }

    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
    }
}

impl I2cSlaveCtrl for I2cSlave
{
    fn listen(&self) -> RetValue<()>
    {
        unsafe { HAL_I2C_EnableListen_IT(self.handle).into() }
    }

    fn transmit(&self, data: &[u8], timeout: u32) -> RetValue<()>
    {
        unsafe { HAL_I2C_Slave_Transmit(self.handle, data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn receive(&self, data: &mut [u8], timeout: u32) -> RetValue<()>
    {
        unsafe { HAL_I2C_Slave_Receive(self.handle, data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn async_transmit(&self, data: &[u8]) -> RetValue<()>
    {
        unsafe { HAL_I2C_Slave_Transmit_DMA(self.handle, data.as_ptr(), data.len() as u16).into() }
    }

    fn async_receive(&self, data: &mut [u8]) -> RetValue<()>
    {
        unsafe { HAL_I2C_Slave_Receive_DMA(self.handle, data.as_ptr(), data.len() as u16).into() }
    }
}

/////////////////////////////////////////////////////////////////////////////
// I2C queue
/////////////////////////////////////////////////////////////////////////////

static mut I2C_QUEUE: SampleQueue<I2c, I2C_HandleTypeDef, I2C_COUNT> = SampleQueue::new();

pub struct I2cQueue;

impl I2cQueue
{
    #[inline]
    #[allow(static_mut_refs)]
    pub fn allocate_mem(sample_handle: *mut I2C_HandleTypeDef) -> RetValue<&'static mut I2cMem>
    {
        if let I2c::Mem(mem) = unsafe { I2C_QUEUE.allocate(&I2c::new_mem(sample_handle)?)? }
        {
            Ok(mem)
        }
        else
        {
            Err(ErrValue::Unknown)
        }
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn allocate_master(sample_handle: *mut I2C_HandleTypeDef) -> RetValue<&'static mut I2cMaster>
    {
        if let I2c::Master(master) = unsafe { I2C_QUEUE.allocate(&I2c::new_master(sample_handle)?)? }
        {
            Ok(master)
        }
        else
        {
            Err(ErrValue::Unknown)
        }
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn allocate_slave(sample_handle: *mut I2C_HandleTypeDef) -> RetValue<&'static mut I2cSlave>
    {
        if let I2c::Slave(slave) = unsafe { I2C_QUEUE.allocate(&I2c::new_slave(sample_handle)?)? }
        {
            Ok(slave)
        }
        else
        {
            Err(ErrValue::Unknown)
        }
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn clean(sample_handle: *mut I2C_HandleTypeDef)
    {
        unsafe { I2C_QUEUE.clean(sample_handle) };
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn search_mem(sample_handle: *mut I2C_HandleTypeDef) -> RetValue<&'static I2cMem>
    {
        if let I2c::Mem(mem) = unsafe { I2C_QUEUE.search(sample_handle)? }
        {
            Ok(mem)
        }
        else
        {
            Err(ErrValue::InstanceNotFound)
        }
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn search_master(sample_handle: *mut I2C_HandleTypeDef) -> RetValue<&'static I2cMaster>
    {
        if let I2c::Master(master) = unsafe { I2C_QUEUE.search(sample_handle)? }
        {
            Ok(master)
        }
        else
        {
            Err(ErrValue::InstanceNotFound)
        }
    }

    #[inline]
    #[allow(static_mut_refs)]
    pub fn search_slave(sample_handle: *mut I2C_HandleTypeDef) -> RetValue<&'static I2cSlave>
    {
        if let I2c::Slave(slave) = unsafe { I2C_QUEUE.search(sample_handle)? }
        {
            Ok(slave)
        }
        else
        {
            Err(ErrValue::InstanceNotFound)
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
// HAL interrupt callback function implementations
/////////////////////////////////////////////////////////////////////////////

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_MasterTxCpltCallback(i2c: *mut I2C_HandleTypeDef)
{
    if let Some(sample) = I2cQueue::search_master(i2c).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_i2c_master_tx_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_MasterRxCpltCallback(i2c: *mut I2C_HandleTypeDef)
{
    if let Some(sample) = I2cQueue::search_master(i2c).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_i2c_master_rx_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_SlaveTxCpltCallback(i2c: *mut I2C_HandleTypeDef)
{
    if let Some(sample) = I2cQueue::search_slave(i2c).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_i2c_slave_tx_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_SlaveRxCpltCallback(i2c: *mut I2C_HandleTypeDef)
{
    if let Some(sample) = I2cQueue::search_slave(i2c).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_i2c_slave_rx_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_AddrCallback(i2c: *mut I2C_HandleTypeDef, transfer_direction: u8, addr_match_code: u16)
{
    if let Some(sample) = I2cQueue::search_slave(i2c).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_i2c_slave_selected(transfer_direction.into(), addr_match_code);
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_ListenCpltCallback(i2c: *mut I2C_HandleTypeDef)
{
    if let Some(sample) = I2cQueue::search_slave(i2c).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_i2c_slave_listen_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_MemTxCpltCallback(i2c: *mut I2C_HandleTypeDef)
{
    if let Some(sample) = I2cQueue::search_mem(i2c).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_i2c_mem_write_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_MemRxCpltCallback(i2c: *mut I2C_HandleTypeDef)
{
    if let Some(sample) = I2cQueue::search_mem(i2c).ok()
    {
        if let Some(event_handle) = sample.event_handle
        {
            (*event_handle).on_i2c_mem_read_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_ErrorCallback(i2c: *mut I2C_HandleTypeDef)
{
    match (*i2c).Mode
    {
        HAL_I2C_ModeTypeDef::None => (),
        HAL_I2C_ModeTypeDef::Master =>
        {
            if let Some(sample) = I2cQueue::search_master(i2c).ok()
            {
                if let Some(event_handle) = sample.event_handle
                {
                    (*event_handle).on_i2c_master_error();
                }
            }
        }
        HAL_I2C_ModeTypeDef::Slave =>
        {
            if let Some(sample) = I2cQueue::search_slave(i2c).ok()
            {
                if let Some(event_handle) = sample.event_handle
                {
                    (*event_handle).on_i2c_slave_error();
                }
            }
        }
        HAL_I2C_ModeTypeDef::Mem =>
        {
            if let Some(sample) = I2cQueue::search_mem(i2c).ok()
            {
                if let Some(event_handle) = sample.event_handle
                {
                    (*event_handle).on_i2c_mem_error();
                }
            }
        }
    }
}

// #[no_mangle]
// #[allow(static_mut_refs)]
// pub unsafe extern "C" fn HAL_I2C_AbortCpltCallback(i2c: *mut I2C_HandleTypeDef) {}
