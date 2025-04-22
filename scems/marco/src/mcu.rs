use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field, Fields, Type};

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
