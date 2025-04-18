use scems::common::result::IResult;
use scems::mcu::common::uart::{Uart, UartEvent};
use scems::mcu::vendors::uart::UartDevice;
use scems::os::common::events::IEvents;
use scems::os::common::mem::IMemCache;
use scems::os::vendors::events::Events;
use scems::os::vendors::mem::MemCache;
use scems::os::vendors::systick_value;

use crate::service::console::{ConsoleCommandDispatches, ConsoleTerminal};

pub enum ConsoleServiceTerminal
{
    None,
    Serial(SerialTerminal),
}

impl ConsoleTerminal for ConsoleServiceTerminal
{
    fn tick<'a>(&mut self, dispatches: &dyn ConsoleCommandDispatches)
    {
        match self
        {
            ConsoleServiceTerminal::None => (),
            ConsoleServiceTerminal::Serial(serial_terminal) => serial_terminal.tick(dispatches),
        };
    }

    fn write_content(&self, _content: &str)
    {
        match self
        {
            ConsoleServiceTerminal::None => (),
            ConsoleServiceTerminal::Serial(serial_terminal) => serial_terminal.write_content(_content),
        };
    }
}

const SERIAL_TRANSMIT_TIMEOUT: u32 = 500;
const SERIAL_MAINWAIT_TIMEOUT: u32 = 10000;

const SERIAL_EVENT_RECEIVED: u32 = 0x01;
const SERIAL_EVENT_ALL: u32 = 0xFF;

pub struct SerialTerminal
{
    uart: UartDevice,
    events: Events,
    input_cache: MemCache<256>,
    output_cache: MemCache<256>,
    receive_size: usize,
}

impl SerialTerminal
{
    pub fn new(uart: UartDevice) -> IResult<Self>
    {
        Ok(Self {
            uart,
            events: Events::new()?,
            input_cache: MemCache::new()?,
            output_cache: MemCache::new()?,
            receive_size: 0,
        })
    }
}

impl ConsoleTerminal for SerialTerminal
{
    fn tick<'a>(&mut self, dispatches: &dyn ConsoleCommandDispatches)
    {
        if let Err(_) = self.uart.async_receive(self.input_cache.as_mut())
        {
            return;
        }

        if let Ok(event) = self.events.receive(SERIAL_EVENT_ALL, SERIAL_MAINWAIT_TIMEOUT)
        {
            if !event.eq(&SERIAL_EVENT_RECEIVED)
            {
                return;
            }

            self.output_cache.clean();

            if let Err(_) = dispatches.dispatch_and_execute(
                self.input_cache.as_ref().split_at(self.receive_size).0,
                self.output_cache.as_mut(),
            )
            {
                self.output_cache.as_mut().copy_from_slice("The command doesn't execute successful.".as_bytes());
            }

            self.input_cache.clean();

            if let Ok(content) = core::str::from_utf8(self.output_cache.as_ref())
            {
                self.write_content(content);
            }
        }
    }

    #[allow(unused_must_use)]
    fn write_content(&self, content: &str)
    {
        let start_time: u32 = systick_value();

        while (systick_value() - start_time) <= SERIAL_TRANSMIT_TIMEOUT
        {
            if let Ok(_) = self.uart.async_transmit(content.as_bytes())
            {
                break;
            }
        }
    }
}

impl UartEvent for SerialTerminal
{
    fn on_uart_rx_complete(&mut self, _size: u32)
    {
        self.receive_size = _size as usize;
        self.events.launch(SERIAL_EVENT_RECEIVED).unwrap_or_default();
    }
}

// impl TaskMain for SerialTerminal
// {
//     fn main(&mut self)
//     {
//         loop
//         {
//             self.input_cache.clean();

//             if let Err(_) = self.uart.async_receive(self.input_cache.as_mut())
//             {
//                 delay(SERIAL_MAINWAIT_TIMEOUT);
//                 continue;
//             }

//             if let Ok(event) = self.events.receive(SERIAL_EVENT_ALL, SERIAL_MAINWAIT_TIMEOUT)
//             {
//                 if !event.eq(&SERIAL_EVENT_RECEIVED)
//                 {
//                     continue;
//                 }

//                 self.output_cache.clean();

//                 if let Some(dispatches) = self.dispatches
//                 {
//                     if let Err(_) = dispatches.dispatch_and_execute(
//                         &self.input_cache.split_at(self.receive_size).0,
//                         self.output_cache.as_mut(),
//                     )
//                     {
//                         self.output_cache
//                             .as_mut()
//                             .copy_from_slice("The command doesn't execute successful.".as_bytes());
//                     }
//                 }

//                 if let Ok(content) = core::str::from_utf8(self.output_cache.as_ref())
//                 {
//                     self.write_content(content);
//                 }
//             }
//         }
//     }
// }

// impl<'a> SerialTerminalTask<'a>
// {
//     pub fn new(uart: UartDevice, task: Task, dispatches: &'a dyn ConsoleCommandDispatches) -> Result<Self>
//     {
//         Ok(Self { task, serial_terminal: SerialTerminal::new(uart, dispatches)? })
//     }
// }
