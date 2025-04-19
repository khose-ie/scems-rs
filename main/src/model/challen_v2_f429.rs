use scems::common::log::LogLevel;
use scems::info;
use scems::mcu::vendors::uart::UartDevice;
use scems::mcu::vendors::UART;
use scems::os::vendors::cmsis::osMemoryPoolId_t;
use scems::os::vendors::mem::{assign_mem_pool, MemPool};
use scems::os::vendors::task::TaskSample;
use scems_addons::service::console::terminal::SerialTerminal;
use scems_addons::service::console::{Console, ConsoleService};

#[allow(improper_ctypes)]
extern "C" {
    static mut huart1: UART;
    static mem_pool0: osMemoryPoolId_t;
    static mem_pool0_block_size: u32;
    static mem_pool1: osMemoryPoolId_t;
    static mem_pool1_block_size: u32;
    static mem_pool2: osMemoryPoolId_t;
    static mem_pool2_block_size: u32;
    static mem_pool3: osMemoryPoolId_t;
    static mem_pool3_block_size: u32;
    static mem_pool4: osMemoryPoolId_t;
    static mem_pool4_block_size: u32;
}

static mut CONSOLE_SERVICE: Option<TaskSample<ConsoleService>> = None;

#[allow(static_mut_refs)]
pub unsafe fn app_main()
{
    initialize_mem_pools();

    CONSOLE_SERVICE = Some(TaskSample::new(ConsoleService::new().unwrap()));
    let console_service = CONSOLE_SERVICE.as_mut().unwrap();

    console_service.assign_serial_terminal(SerialTerminal::new(UartDevice::new(&mut huart1)).unwrap()).unwrap();
    scems::common::log::assign_stream(console_service.as_mut());
    scems::common::log::set_level(LogLevel::Debug);

    info!("     ___  ___ ___ _ __ ___  ___ \r");
    info!("    / __|/ __/ _ \\ '_ ` _ \\/ __|\r");
    info!("    \\__ \\ (_|  __/ | | | | \\__ \\\r");
    info!("    |___/\\___\\___|_| |_| |_|___/\r");
    info!("");
}

pub unsafe fn initialize_mem_pools()
{
    assign_mem_pool(&MemPool::from(mem_pool0, mem_pool0_block_size)).unwrap();
    assign_mem_pool(&MemPool::from(mem_pool1, mem_pool1_block_size)).unwrap();
    assign_mem_pool(&MemPool::from(mem_pool2, mem_pool2_block_size)).unwrap();
    assign_mem_pool(&MemPool::from(mem_pool3, mem_pool3_block_size)).unwrap();
    assign_mem_pool(&MemPool::from(mem_pool4, mem_pool4_block_size)).unwrap();
}
