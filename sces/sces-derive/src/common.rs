use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Item};

pub fn derive_as_ptr(input: TokenStream) -> TokenStream
{
    let input = parse_macro_input!(input as Item);
    let item = match input
    {
        Item::Struct(item_struct) => item_struct,
        _ => panic!("PtrImplement only supports struct."),
    };

    let name = &item.ident;
    let generics = &item.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {

        impl #impl_generics #name #ty_generics #where_clause
        {
            #[inline]
            fn as_ptr(&self) -> *mut #name
            {
                self as *const #name as *mut #name
            }
        }
    };

    TokenStream::from(expanded)
}