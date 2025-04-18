mod common;
mod enum_marco;
mod mcu;

use proc_macro::TokenStream;
use quote::quote;

use crate::common::*;
use crate::enum_marco::*;
use crate::mcu::*;

#[proc_macro_derive(EnumCount)]
pub fn enum_count(input: TokenStream) -> TokenStream
{
    derive_enum_count(input)
}

#[proc_macro_derive(EnumCastU8)]
pub fn enum_cast_u8(input: TokenStream) -> TokenStream
{
    derive_enum_cast(input, quote! { u8 })
}

#[proc_macro_derive(EnumCastU16)]
pub fn enum_cast_u16(input: TokenStream) -> TokenStream
{
    derive_enum_cast(input, quote! { u16 })
}

#[proc_macro_derive(EnumCastU32)]
pub fn enum_cast_u32(input: TokenStream) -> TokenStream
{
    derive_enum_cast(input, quote! { u32 })
}

#[proc_macro_derive(EnumCastI32)]
pub fn enum_cast_i32(input: TokenStream) -> TokenStream
{
    derive_enum_cast(input, quote! { i32 })
}

#[proc_macro_derive(EnumCastUsize)]
pub fn enum_cast_usize(input: TokenStream) -> TokenStream
{
    derive_enum_cast(input, quote! { usize })
}

#[proc_macro_derive(AsPtr)]
pub fn as_ptr(input: TokenStream) -> TokenStream
{
    derive_as_ptr(input)
}

#[proc_macro_derive(HandlePtr)]
pub fn handle_ptr(input: TokenStream) -> TokenStream
{
    derive_handle_ptr(input)
}

#[proc_macro_derive(AsIoEventPtr)]
pub fn as_io_event_ptr(input: TokenStream) -> TokenStream
{
    derive_as_event_ptr(input, quote! { IoEvent })
}

#[proc_macro_derive(AsUartEventPtr)]
pub fn as_uart_event_ptr(input: TokenStream) -> TokenStream
{
    derive_as_event_ptr(input, quote! { UartEvent })
}
