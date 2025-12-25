use log::info;
use log::LevelFilter;
use sces::cell::StaticCell;
use sces::os::mem::MemZone;
use sces::os::task::{TaskPriority, TaskSample};
use sces::value::RetValue;
use sces_mcu_stm32::uart::{UART_HandleTypeDef, UartQueue};
use sces_mcu_stm32::wd::{IWDG_HandleTypeDef, WatchDogQueue};
use sces_os_cmsis::mem::initialize_mem_space;
use sces_os_cmsis::task::Task;
use sces_os_cmsis::CMSISOS;
use sces_svc_alive::{AliveWatchService, NativeAliveWatch};
use sces_svc_console::{ConsoleService, NativeConsole};

#[allow(improper_ctypes)]
extern "C" {
    static mut huart1: UART_HandleTypeDef;
    static mut hwdt1: IWDG_HandleTypeDef;
}

#[global_allocator]
static mut MEM: MemorySpace<CMSISOS, 256, 10, 512, 10, 1024, 10, 2048, 2> = MemorySpace::new();

static mut SVC_CONSOLE: StaticCell<TaskSample<CMSISOS, NativeConsole<CMSISOS>>> = StaticCell::new();
static mut SVC_ALIVE: StaticCell<TaskSample<CMSISOS, NativeAliveWatch<CMSISOS>>> = StaticCell::new();

#[allow(static_mut_refs)]
pub unsafe fn app_main() -> RetValue<()>
{
    MEM.initialize()?;
    CMSISOS::initialize()?;

    SVC_ALIVE
        .set(TaskSample::new(NativeAliveWatch::new(WatchDogQueue::alloc(&mut hwdt1)?, 300)?)?)
        .and_then(|x| x.active("AliveWatchService", 1024, TaskPriority::High))
        .and_then(|x| AliveWatchService::initialize(x.as_ref()))?;

    SVC_CONSOLE
        .set(TaskSample::new(NativeConsole::new(UartQueue::alloc(&mut huart1)?)?)?)
        .and_then(|x| x.active("ConsoleService", 1024, TaskPriority::Normal))
        .and_then(|x| ConsoleService::initialize(x.as_ref(), LevelFilter::Info))?;

    app_print_trademark();

    Ok(())
}

fn app_print_trademark()
{
    info!("     ___  ___ ___ _ __ ___  ___ \r");
    info!("    / __|/ __/ _ \\ '_ ` _ \\/ __|\r");
    info!("    \\__ \\ (_|  __/ | | | | \\__ \\\r");
    info!("    |___/\\___\\___|_| |_| |_|___/\r");
    info!("");
}
