use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, Fields, Item, Type};

pub fn derive_handle_ptr(input: TokenStream) -> TokenStream
{
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: Fields::Named(ref fields),
        ..
    }) = input.data
    {
        fields.clone()
    }
    else
    {
        panic!("HandlePtr can only be derived for structs with named fields");
    };

    let handle = fields
        .named
        .iter()
        .find(|f| f.ident.as_ref().map(|ident| ident == "handle").unwrap_or(false));

    let handle_type = match handle
    {
        Some(Field { ty, .. }) =>
        {
            if let Type::Ptr(syn::TypePtr { elem, .. }) = ty
            {
                *elem.clone()
            }
            else
            {
                panic!("The handle field must be a pointer type (*mut T)");
            }
        }
        None => panic!("struct must have a field named 'handle'"),
    };

    let expanded = quote! {

        impl #impl_generics HandlePtr<#handle_type> for #name #ty_generics #where_clause
        {
            #[inline]
            fn handle_ptr(&self) -> *mut #handle_type
            {
                self.handle as *const #handle_type as *mut #handle_type
            }
        }
    };

    TokenStream::from(expanded)
}

pub fn derive_as_event_ptr(input: TokenStream, event: proc_macro2::TokenStream) -> TokenStream
{
    let input = parse_macro_input!(input as Item);
    let item = match input
    {
        Item::Struct(item_struct) => item_struct,
        _ => panic!("EventImplement only supports struct and trait with default implementations."),
    };

    let name = &item.ident;
    let generics = &item.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {

        impl #impl_generics #event for #name #ty_generics #where_clause {}

        impl #impl_generics AsEventPtr<dyn #event> for #name #ty_generics #where_clause
        {
            #[inline]
            fn as_event_ptr(&self) -> *mut dyn #event
            {
                self as *const dyn #event as *mut dyn #event
            }
        }
    };

    TokenStream::from(expanded)
}
