use core::mem::{transmute, ManuallyDrop};

use crate::common::result::IResult;
use crate::derive::{AsPtr, HandlePtr};
use crate::mcu::common::i2c::{I2cMaster, I2cMasterEventAgent};
use crate::mcu::common::i2c::{I2cMem, I2cMemEventAgent, I2cMemWide};
use crate::mcu::common::i2c::{I2cSlave, I2cSlaveEventAgent};
use crate::mcu::common::{EventLaunch, HandlePtr};
use crate::mcu::vendors::stm::common::DeviceQueue;
use crate::mcu::vendors::stm::native::i2c::*;

const I2C_COUNT: usize = 8;
static mut I2CS: DeviceQueue<I2C, I2cDevice, I2C_COUNT> = DeviceQueue::new();

#[repr(C)]
union I2cDevice
{
    pub mem: ManuallyDrop<I2cMemDevice>,
    pub master: ManuallyDrop<I2cMasterDevice>,
    pub slave: ManuallyDrop<I2cSlaveDevice>,
}

impl HandlePtr<I2C> for I2cDevice
{
    #[inline]
    fn handle_ptr(&self) -> *mut I2C
    {
        unsafe { self.mem.handle_ptr() }
    }
}

#[derive(AsPtr, HandlePtr)]
pub struct I2cMemDevice
{
    handle: *mut I2C,
    event_handle: Option<*const dyn I2cMemEventAgent>,
}

impl I2cMemDevice
{
    pub fn new(handle: *mut I2C) -> Self
    {
        I2cMemDevice { handle, event_handle: None }
    }

    fn as_i2c_ptr(&self) -> *mut I2cDevice
    {
        self as *const I2cMemDevice as *mut I2cMemDevice as *mut I2cDevice
    }
}

impl Drop for I2cMemDevice
{
    fn drop(&mut self)
    {
        self.clean_event_agent();
    }
}

impl EventLaunch<dyn I2cMemEventAgent> for I2cMemDevice
{
    #[allow(static_mut_refs)]
    fn set_event_agent(&mut self, event_handle: &dyn I2cMemEventAgent) -> IResult<()>
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn I2cMemEventAgent) });
        unsafe { I2CS.alloc(self.as_i2c_ptr()) }
    }

    #[allow(static_mut_refs)]
    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
        unsafe { I2CS.clean(self.as_i2c_ptr()) };
    }
}

impl I2cMem for I2cMemDevice
{
    fn mem_write(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &[u8], timeout: u32) -> IResult<()>
    {
        unsafe {
            HAL_I2C_Mem_Write(self.handle, saddr, maddr, mwide.into(), data.as_ptr(), data.len() as u16, timeout).into()
        }
    }

    fn mem_read(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &mut [u8], timeout: u32) -> IResult<()>
    {
        unsafe {
            HAL_I2C_Mem_Read(self.handle, saddr, maddr, mwide.into(), data.as_ptr(), data.len() as u16, timeout).into()
        }
    }

    fn async_mem_write(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &[u8]) -> IResult<()>
    {
        unsafe {
            HAL_I2C_Mem_Write_DMA(self.handle, saddr, maddr, mwide.into(), data.as_ptr(), data.len() as u16).into()
        }
    }

    fn async_mem_read(&self, saddr: u16, maddr: u16, mwide: I2cMemWide, data: &mut [u8]) -> IResult<()>
    {
        unsafe {
            HAL_I2C_Mem_Read_DMA(self.handle, saddr, maddr, mwide.into(), data.as_ptr(), data.len() as u16).into()
        }
    }
}

#[derive(AsPtr, HandlePtr)]
pub struct I2cMasterDevice
{
    handle: *mut I2C,
    event_handle: Option<*const dyn I2cMasterEventAgent>,
}

impl I2cMasterDevice
{
    pub fn new(handle: *mut I2C) -> Self
    {
        I2cMasterDevice { handle, event_handle: None }
    }

    fn as_i2c_ptr(&self) -> *mut I2cDevice
    {
        self as *const I2cMasterDevice as *mut I2cMasterDevice as *mut I2cDevice
    }
}

impl Drop for I2cMasterDevice
{
    fn drop(&mut self)
    {
        self.clean_event_agent();
    }
}

impl EventLaunch<dyn I2cMasterEventAgent> for I2cMasterDevice
{
    #[allow(static_mut_refs)]
    fn set_event_agent(&mut self, event_handle: &dyn I2cMasterEventAgent) -> IResult<()>
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn I2cMasterEventAgent) });
        unsafe { I2CS.alloc(self.as_i2c_ptr()) }
    }

    #[allow(static_mut_refs)]
    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
        unsafe { I2CS.clean(self.as_i2c_ptr()) };
    }
}

impl I2cMaster for I2cMasterDevice
{
    fn transmit(&self, saddr: u16, data: &[u8], timeout: u32) -> IResult<()>
    {
        unsafe { HAL_I2C_Master_Transmit(self.handle, saddr, data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn receive(&self, saddr: u16, data: &mut [u8], timeout: u32) -> IResult<()>
    {
        unsafe { HAL_I2C_Master_Receive(self.handle, saddr, data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn async_transmit(&self, saddr: u16, data: &[u8]) -> IResult<()>
    {
        unsafe { HAL_I2C_Master_Transmit_DMA(self.handle, saddr, data.as_ptr(), data.len() as u16).into() }
    }

    fn async_receive(&self, saddr: u16, data: &mut [u8]) -> IResult<()>
    {
        unsafe { HAL_I2C_Master_Receive_DMA(self.handle, saddr, data.as_ptr(), data.len() as u16).into() }
    }
}

#[derive(AsPtr, HandlePtr)]
pub struct I2cSlaveDevice
{
    handle: *mut I2C,
    event_handle: Option<*const dyn I2cSlaveEventAgent>,
}

impl I2cSlaveDevice
{
    pub fn new(handle: *mut I2C) -> Self
    {
        I2cSlaveDevice { handle, event_handle: None }
    }

    fn as_i2c_ptr(&self) -> *mut I2cDevice
    {
        self as *const I2cSlaveDevice as *mut I2cSlaveDevice as *mut I2cDevice
    }
}

impl Drop for I2cSlaveDevice
{
    fn drop(&mut self)
    {
        self.clean_event_agent();
    }
}

impl EventLaunch<dyn I2cSlaveEventAgent> for I2cSlaveDevice
{
    #[allow(static_mut_refs)]
    fn set_event_agent(&mut self, event_handle: &dyn I2cSlaveEventAgent) -> IResult<()>
    {
        self.event_handle = Some(unsafe { transmute(event_handle as *const dyn I2cSlaveEventAgent) });
        unsafe { I2CS.alloc(self.as_i2c_ptr()) }
    }

    #[allow(static_mut_refs)]
    fn clean_event_agent(&mut self)
    {
        self.event_handle = None;
        unsafe { I2CS.clean(self.as_i2c_ptr()) };
    }
}

impl I2cSlave for I2cSlaveDevice
{
    fn listen(&self) -> IResult<()>
    {
        unsafe { HAL_I2C_EnableListen_IT(self.handle).into() }
    }

    fn transmit(&self, data: &[u8], timeout: u32) -> IResult<()>
    {
        unsafe { HAL_I2C_Slave_Transmit(self.handle, data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn receive(&self, data: &mut [u8], timeout: u32) -> IResult<()>
    {
        unsafe { HAL_I2C_Slave_Receive(self.handle, data.as_ptr(), data.len() as u16, timeout).into() }
    }

    fn async_transmit(&self, data: &[u8]) -> IResult<()>
    {
        unsafe { HAL_I2C_Slave_Transmit_DMA(self.handle, data.as_ptr(), data.len() as u16).into() }
    }

    fn async_receive(&self, data: &mut [u8]) -> IResult<()>
    {
        unsafe { HAL_I2C_Slave_Receive_DMA(self.handle, data.as_ptr(), data.len() as u16).into() }
    }
}

#[no_mangle]
#[allow(static_mut_refs)]
pub unsafe extern "C" fn HAL_I2C_MasterTxCpltCallback(i2c: *mut I2C)
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
pub unsafe extern "C" fn HAL_I2C_MasterRxCpltCallback(i2c: *mut I2C)
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
pub unsafe extern "C" fn HAL_I2C_SlaveTxCpltCallback(i2c: *mut I2C)
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
pub unsafe extern "C" fn HAL_I2C_SlaveRxCpltCallback(i2c: *mut I2C)
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
pub unsafe extern "C" fn HAL_I2C_AddrCallback(i2c: *mut I2C, transfer_direction: u8, addr_match_code: u16)
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
pub unsafe extern "C" fn HAL_I2C_ListenCpltCallback(i2c: *mut I2C)
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
pub unsafe extern "C" fn HAL_I2C_MemTxCpltCallback(i2c: *mut I2C)
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
pub unsafe extern "C" fn HAL_I2C_MemRxCpltCallback(i2c: *mut I2C)
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
pub unsafe extern "C" fn HAL_I2C_ErrorCallback(i2c: *mut I2C)
{
    match (*i2c).Mode
    {
        HAL_I2C_Mode::None => (),
        HAL_I2C_Mode::Master =>
        {
            if let Some(sample) = I2CS.find(i2c).ok()
            {
                if let Some(event_handle) = (*sample).master.event_handle
                {
                    (*event_handle).on_i2c_master_error();
                }
            }
        }
        HAL_I2C_Mode::Slave =>
        {
            if let Some(sample) = I2CS.find(i2c).ok()
            {
                if let Some(event_handle) = (*sample).slave.event_handle
                {
                    (*event_handle).on_i2c_slave_error();
                }
            }
        }
        HAL_I2C_Mode::Mem =>
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
// pub unsafe extern "C" fn HAL_I2C_AbortCpltCallback(i2c: *mut I2C) {}
