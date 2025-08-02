use core::cell::RefCell;

use scems::common::result::RetValue;
use scems::mcu::common::uart::{Uart, UartEventAgent};
use scems::mcu::vendor::uart::UartDevice;
use scems::os::common::events::IEvents;
use scems::os::common::mem::IMemCache;
use scems::os::vendors::events::Events;
use scems::os::vendors::mem::MemCache;

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

    fn async_write_content(&self, _content: &str)
    {
        match self
        {
            ConsoleServiceTerminal::None => (),
            ConsoleServiceTerminal::Serial(serial_terminal) => serial_terminal.async_write_content(_content),
        };
    }
}

const TX_TIMEOUT: u32 = 500;
const RX_TIMEOUT: u32 = 10000;

const EVENT_TX_COMPLETE: u32 = 0x01;
const EVENT_RX_COMPLETE: u32 = 0x01;

pub struct SerialTerminal
{
    uart: UartDevice,
    events: Events,
    input_cache: MemCache<256>,
    output_cache: MemCache<256>,
    receive_size: RefCell<usize>,
}

impl SerialTerminal
{
    pub fn new(uart: UartDevice) -> RetValue<Self>
    {
        Ok(Self {
            uart,
            events: Events::new()?,
            input_cache: MemCache::new()?,
            output_cache: MemCache::new()?,
            receive_size: RefCell::new(0),
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

        if let Ok(_) = self.events.receive(EVENT_RX_COMPLETE, RX_TIMEOUT)
        {
            self.output_cache.clean();

            if let Err(_) = dispatches.dispatch_and_execute(
                self.input_cache.as_ref().split_at(*self.receive_size.borrow_mut()).0,
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
        self.uart.transmit(content.as_bytes(), TX_TIMEOUT);
    }

    #[allow(unused_must_use)]
    fn async_write_content(&self, _content: &str)
    {
        if let Ok(_) = self.events.receive(EVENT_TX_COMPLETE, TX_TIMEOUT)
        {
            self.uart.async_transmit(_content.as_bytes());
        }
    }
}

impl UartEventAgent for SerialTerminal
{
    fn on_uart_tx_complete(&self)
    {
        self.events.launch(EVENT_TX_COMPLETE).unwrap_or_default();
    }

    fn on_uart_rx_complete(&self, _size: u32)
    {
        if let Ok(mut receive_size) = self.receive_size.try_borrow_mut()
        {
            *receive_size = _size as usize;
            self.events.launch(EVENT_RX_COMPLETE).unwrap_or_default();
        }
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
//                 delay(RX_TIMEOUT);
//                 continue;
//             }

//             if let Ok(event) = self.events.receive(EVENT_ALL, RX_TIMEOUT)
//             {
//                 if !event.eq(&EVENT_RX_COMPLETE)
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
