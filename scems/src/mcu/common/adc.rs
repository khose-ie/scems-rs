//! Provide a common trait to operate the Analog-Digital Converter (ADC).

use super::EventLaunch;
use crate::common::result::RetValue;

/// `AdcDevice` is the common trait for ADC peripheral, provides the interface functions to operate the
/// ADC.
/// 
/// The ADC implementations for every MCU manufacturers should implement this trait.
/// And every upper modules who want to control an ADC should reference this trait, and call the
/// functions of this trait.
pub trait AdcDevice
where
    Self: EventLaunch<dyn AdcDeviceEventAgent>,
{
    /// Convert the analog signal to an unsigned 32-bit integer once.
    ///
    /// The integer will reflect the situation of the ADC when calling this function (there may be
    /// a little delay).
    fn convert_once(&self) -> RetValue<u32>;

    /// Convert the analog signal to an unsigned 32-bit integer once. But don't return the value
    /// immediatelay via the return value.
    ///
    /// You call this function, means that you have enable the interrupts of the related ADC in
    /// the low-level platform initialization code.
    /// The converting value will be transport the interrupt handle function of the MCU, and the
    /// caller should register an event agent to this `AdcDevice`, see [`super::EventLaunch`]
    fn async_convert_once(&self) -> RetValue<()>;

    /// To start the continuous conversion of an ADC.
    /// 
    /// You need to transport a slice of an array, the ADC will convert the analog to digital 
    /// value and put the value to the slice you input.
    /// The ADC will not use the space over the scope of your slice, it will rotate the space.
    /// The coversion will always continued aftet you call this function, and the value in `data` 
    /// space will be updated until you stop it, see [`AdcDevice::async_terminate_conversion`].
    fn async_convert_continuous(&self, data: &mut [u32]) -> RetValue<()>;

    /// To stop the continuous conversion of an ADC.
    /// 
    /// When you don't need the ADC, or you don't want it continue the work, you should call this 
    /// function to stop the conversion action.
    /// After you call this function, the values in the `data` space, which is a slice you inputed 
    /// when you call function `async_convert_continuous`, will not be updated, but the current 
    /// value will not be cleared or changed.
    fn async_terminate_conversion(&self) -> RetValue<()>;
}

/// `AdcDeviceEventAgent` as the meanings of the word, it is an agent, or the real handler, 
/// whatever, to handle all events sent from the `Adc`.
/// 
/// Actually, these callback functions will be called in the interrupt vector handle which 
/// triggered by ADC peripheral.
/// Please attention, don't use any waiting and hang actions in these functions, because it will 
/// block the system interrupts.
/// 
/// All functions of this trait have an empty default implementation, it meanus that you can only 
/// implement the function that you care about.
pub trait AdcDeviceEventAgent
{
    /// Will be called when the once conversion action has been completed.
    /// 
    /// This function will be called after you call [`AdcDevice::async_convert_once`], and the 
    /// converted value will be transported via the parameter `_value`.
    fn on_adc_convert_once_complete(&self, _value: u32) {}

    /// This function is use for the watch feature of the ADC, when the value is out of the window 
    /// you set in the initialization code, this function will be called.
    fn on_adc_level_out_of_window(&self) {}

    /// Will be called when the ADC peripheral has some errors.
    fn on_adc_error(&self) {}
}
