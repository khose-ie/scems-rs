// mod common;
mod types;
// mod mcu;

use proc_macro::TokenStream;
use quote::quote;

// use crate::common::*;
use crate::types::enums::*;
// use crate::mcu::*;

#[proc_macro_derive(EnumCount)]
pub fn enum_count(input: TokenStream) -> TokenStream
{
    derive_enum_count(input)
}

#[proc_macro_derive(EnumAsU8)]
pub fn enum_cast_u8(input: TokenStream) -> TokenStream
{
    derive_enum_cast(input, quote! { u8 })
}

#[proc_macro_derive(EnumAsU16)]
pub fn enum_cast_u16(input: TokenStream) -> TokenStream
{
    derive_enum_cast(input, quote! { u16 })
}

#[proc_macro_derive(EnumAsU32)]
pub fn enum_cast_u32(input: TokenStream) -> TokenStream
{
    derive_enum_cast(input, quote! { u32 })
}

#[proc_macro_derive(EnumAsUsize)]
pub fn enum_cast_usize(input: TokenStream) -> TokenStream
{
    derive_enum_cast(input, quote! { usize })
}

#[proc_macro_derive(EnumAsI8)]
pub fn enum_cast_i8(input: TokenStream) -> TokenStream
{
    derive_enum_cast(input, quote! { i8 })
}

#[proc_macro_derive(EnumAsI16)]
pub fn enum_cast_i16(input: TokenStream) -> TokenStream
{
    derive_enum_cast(input, quote! { i16 })
}

#[proc_macro_derive(EnumAsI32)]
pub fn enum_cast_i32(input: TokenStream) -> TokenStream
{
    derive_enum_cast(input, quote! { i32 })
}

#[proc_macro_derive(EnumAsIsize)]
pub fn enum_cast_isize(input: TokenStream) -> TokenStream
{
    derive_enum_cast(input, quote! { isize })
}

// #[proc_macro_derive(AsPtr)]
// pub fn as_ptr(input: TokenStream) -> TokenStream
// {
//     derive_as_ptr(input)
// }

// #[proc_macro_derive(HandlePtr)]
// pub fn handle_ptr(input: TokenStream) -> TokenStream
// {
//     derive_handle_ptr(input)
// }
