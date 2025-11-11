use log::{info, LevelFilter};
use scems::{
    cell::StaticCell,
    value::{ErrValue, RetValue},
};
use scems_mcu_stm32::{
    uart::{UART_HandleTypeDef, UartQueue},
    wd::{IWDG_HandleTypeDef, WatchDog, WatchDogQueue},
};
use scems_os::task::TaskSample;
use scems_os_cmsis::{events::Events, mutex::Mutex, task::Task, CMSISOS};
use scems_svc_alive::{AliveWatchService, NativeAliveWatch};
use scems_svc_console::{ConsoleService, NativeConsole};

#[allow(improper_ctypes)]
extern "C" {
    static mut huart1: UART_HandleTypeDef;
    static mut hwdt1: IWDG_HandleTypeDef;
    // static mem_pool0: osMemoryPoolId_t;
    // static mem_pool0_block_size: u32;
    // static mem_pool1: osMemoryPoolId_t;
    // static mem_pool1_block_size: u32;
    // static mem_pool2: osMemoryPoolId_t;
    // static mem_pool2_block_size: u32;
    // static mem_pool3: osMemoryPoolId_t;
    // static mem_pool3_block_size: u32;
    // static mem_pool4: osMemoryPoolId_t;
    // static mem_pool4_block_size: u32;
}

static mut SVC_CONSOLE: StaticCell<TaskSample<Task, NativeConsole<CMSISOS>>> = StaticCell::new();
static mut SVC_ALIVE: StaticCell<TaskSample<Task, NativeAliveWatch<CMSISOS>>> = StaticCell::new();

#[allow(static_mut_refs)]
pub unsafe fn app_main() -> RetValue<()>
{
    initialize_mem_pools();

    #[rustfmt::skip]
    SVC_CONSOLE
        .set(TaskSample::new(Task::new(),
                             NativeConsole::new(UartQueue::alloc(&mut huart1)?, Events::new()?,
                                                Mutex::new()?, Mutex::new()?)))
        .and_then(|x| ConsoleService::initialize(x.as_ref(), LevelFilter::Info))?;

    #[rustfmt::skip]
    SVC_ALIVE
        .set(TaskSample::new(Task::new(),
                             NativeAliveWatch::new(WatchDogQueue::alloc(&mut hwdt1)?, Mutex::new()?, 300)?))
        .and_then(|x| AliveWatchService::initialize(x.as_ref()))?;

    info!("     ___  ___ ___ _ __ ___  ___ \r");
    info!("    / __|/ __/ _ \\ '_ ` _ \\/ __|\r");
    info!("    \\__ \\ (_|  __/ | | | | \\__ \\\r");
    info!("    |___/\\___\\___|_| |_| |_|___/\r");
    info!("");
    Ok(())
}

pub unsafe fn initialize_mem_pools()
{
    // assign_mem_pool(&MemPool::from(mem_pool0, mem_pool0_block_size)).unwrap();
    // assign_mem_pool(&MemPool::from(mem_pool1, mem_pool1_block_size)).unwrap();
    // assign_mem_pool(&MemPool::from(mem_pool2, mem_pool2_block_size)).unwrap();
    // assign_mem_pool(&MemPool::from(mem_pool3, mem_pool3_block_size)).unwrap();
    // assign_mem_pool(&MemPool::from(mem_pool4, mem_pool4_block_size)).unwrap();
}
