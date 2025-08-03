use core::mem::{transmute, ManuallyDrop};

use crate::common::result::RetValue;
use crate::derive::{AsPtr, HandlePtr};
use crate::mcu::common::i2c::{I2cMasterDevice, I2cMasterDeviceEventAgent};
use crate::mcu::common::i2c::{I2cMemDevice, I2cMemDeviceEventAgent, I2cMemWide};
use crate::mcu::common::i2c::{I2cSlaveDevice, I2cSlaveDeviceEventAgent};
use crate::mcu::common::{EventLaunch, HandlePtr};
use crate::mcu::vendor::stm::device_queue::DeviceQueue;
use crate::mcu::vendor::stm::native::i2c::*;

pub use crate::mcu::vendor::stm::native::i2c::I2C_HandleTypeDef;

const I2C_COUNT: usize = 8;
static mut I2CS: DeviceQueue<I2C_HandleTypeDef, I2c, I2C_COUNT> = DeviceQueue::new();

#[repr(C)]
union I2c
{
    pub mem: ManuallyDrop<I2cMem>,
    pub master: ManuallyDrop<I2cMaster>,
    pub slave: ManuallyDrop<I2cSlave>,
}

impl HandlePtr<I2C_HandleTypeDef> for I2c
{
    #[inline]
    fn handle_ptr(&self) -> *mut I2C_HandleTypeDef
    {
        unsafe { self.mem.handle_ptr() }
    }
}

#[derive(AsPtr, HandlePtr)]
pub struct I2cMem
{
    handle: *mut I2C_HandleTypeDef,
    event_handle: Option<*const dyn I2cMemDeviceEventAgent>,
}

impl I2cMem
{
    pub fn new(handle: *mut I2C_HandleTypeDef) -> Self
    {
        I2cMem { handle, event_handle: None }
    }

    fn as_i2c_ptr(&self) -> *mut I2c
    {
        self as *const I2cMem as *mut I2cMem as *mut I2c
    }
}

impl Drop for I2cMem
{
    fn drop(&mut self)
    {
        self.clean_event_agent();
    }
}

impl EventLaunch<dyn I2cMemDeviceEventAgent> for I2cMem
{
    #[allow(static_mut_refs)]
    fn set_event_agent(&mut self, event_handle: &dyn I2cMemDeviceEventAgent) -> RetValue<()>
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn I2cMemDeviceEventAgent) });
        unsafe { I2CS.alloc(self.as_i2c_ptr()) }
    }

    #[allow(static_mut_refs)]
    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
        unsafe { I2CS.clean(self.as_i2c_ptr()) };
    }
}

impl I2cMemDevice for I2cMem
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

#[derive(AsPtr, HandlePtr)]
pub struct I2cMaster
{
    handle: *mut I2C_HandleTypeDef,
    event_handle: Option<*const dyn I2cMasterDeviceEventAgent>,
}

impl I2cMaster
{
    pub fn new(handle: *mut I2C_HandleTypeDef) -> Self
    {
        I2cMaster { handle, event_handle: None }
    }

    fn as_i2c_ptr(&self) -> *mut I2c
    {
        self as *const I2cMaster as *mut I2cMaster as *mut I2c
    }
}

impl Drop for I2cMaster
{
    fn drop(&mut self)
    {
        self.clean_event_agent();
    }
}

impl EventLaunch<dyn I2cMasterDeviceEventAgent> for I2cMaster
{
    #[allow(static_mut_refs)]
    fn set_event_agent(&mut self, event_handle: &dyn I2cMasterDeviceEventAgent) -> RetValue<()>
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn I2cMasterDeviceEventAgent) });
        unsafe { I2CS.alloc(self.as_i2c_ptr()) }
    }

    #[allow(static_mut_refs)]
    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
        unsafe { I2CS.clean(self.as_i2c_ptr()) };
    }
}

impl I2cMasterDevice for I2cMaster
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

#[derive(AsPtr, HandlePtr)]
pub struct I2cSlave
{
    handle: *mut I2C_HandleTypeDef,
    event_handle: Option<*const dyn I2cSlaveDeviceEventAgent>,
}

impl I2cSlave
{
    pub fn new(handle: *mut I2C_HandleTypeDef) -> Self
    {
        I2cSlave { handle, event_handle: None }
    }

    fn as_i2c_ptr(&self) -> *mut I2c
    {
        self as *const I2cSlave as *mut I2cSlave as *mut I2c
    }
}

impl Drop for I2cSlave
{
    fn drop(&mut self)
    {
        self.clean_event_agent();
    }
}

impl EventLaunch<dyn I2cSlaveDeviceEventAgent> for I2cSlave
{
    #[allow(static_mut_refs)]
    fn set_event_agent(&mut self, event_handle: &dyn I2cSlaveDeviceEventAgent) -> RetValue<()>
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn I2cSlaveDeviceEventAgent) });
        unsafe { I2CS.alloc(self.as_i2c_ptr()) }
    }

    #[allow(static_mut_refs)]
    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
        unsafe { I2CS.clean(self.as_i2c_ptr()) };
    }
}

impl I2cSlaveDevice for I2cSlave
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

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_MasterTxCpltCallback(i2c: *mut I2C_HandleTypeDef)
{
    if let Some(sample) = I2CS.find(i2c).ok()
    {
        if let Some(event_handle) = (*sample).master.event_handle
        {
            (*event_handle).on_i2c_master_tx_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_MasterRxCpltCallback(i2c: *mut I2C_HandleTypeDef)
{
    if let Some(sample) = I2CS.find(i2c).ok()
    {
        if let Some(event_handle) = (*sample).master.event_handle
        {
            (*event_handle).on_i2c_master_rx_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_SlaveTxCpltCallback(i2c: *mut I2C_HandleTypeDef)
{
    if let Some(sample) = I2CS.find(i2c).ok()
    {
        if let Some(event_handle) = (*sample).slave.event_handle
        {
            (*event_handle).on_i2c_slave_tx_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_SlaveRxCpltCallback(i2c: *mut I2C_HandleTypeDef)
{
    if let Some(sample) = I2CS.find(i2c).ok()
    {
        if let Some(event_handle) = (*sample).slave.event_handle
        {
            (*event_handle).on_i2c_slave_rx_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_AddrCallback(i2c: *mut I2C_HandleTypeDef, transfer_direction: u8, addr_match_code: u16)
{
    if let Some(sample) = I2CS.find(i2c).ok()
    {
        if let Some(event_handle) = (*sample).slave.event_handle
        {
            (*event_handle).on_i2c_slave_selected(transfer_direction.into(), addr_match_code);
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_ListenCpltCallback(i2c: *mut I2C_HandleTypeDef)
{
    if let Some(sample) = I2CS.find(i2c).ok()
    {
        if let Some(event_handle) = (*sample).slave.event_handle
        {
            (*event_handle).on_i2c_slave_listen_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_MemTxCpltCallback(i2c: *mut I2C_HandleTypeDef)
{
    if let Some(sample) = I2CS.find(i2c).ok()
    {
        if let Some(event_handle) = (*sample).mem.event_handle
        {
            (*event_handle).on_i2c_mem_write_complete();
        }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_MemRxCpltCallback(i2c: *mut I2C_HandleTypeDef)
{
    if let Some(sample) = I2CS.find(i2c).ok()
    {
        if let Some(event_handle) = (*sample).mem.event_handle
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
            if let Some(sample) = I2CS.find(i2c).ok()
            {
                if let Some(event_handle) = (*sample).master.event_handle
                {
                    (*event_handle).on_i2c_master_error();
                }
            }
        }
        HAL_I2C_ModeTypeDef::Slave =>
        {
            if let Some(sample) = I2CS.find(i2c).ok()
            {
                if let Some(event_handle) = (*sample).slave.event_handle
                {
                    (*event_handle).on_i2c_slave_error();
                }
            }
        }
        HAL_I2C_ModeTypeDef::Mem =>
        {
            if let Some(sample) = I2CS.find(i2c).ok()
            {
                if let Some(event_handle) = (*sample).mem.event_handle
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
