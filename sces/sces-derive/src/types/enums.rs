use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput};

pub fn derive_enum_cast(input: TokenStream, repr_ty: proc_macro2::TokenStream) -> TokenStream
{
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let variants = match &input.data
    {
        Data::Enum(data_enum) => &data_enum.variants,
        _ => panic!("EnumCast only supports enums."),
    };

    let match_arms = variants.iter().map(|v| {
        let ident = &v.ident;
        let discriminant = &v.discriminant.as_ref().expect("Must use explicit discriminant.").1;
        quote! {
            #discriminant => Self::#ident,
        }
    });

    let fallback = &variants.last().expect("Enum must have at least one variant。").ident;

    let expanded = quote! {

        impl From<#repr_ty> for #enum_name
        {
            fn from(value: #repr_ty) -> Self
            {
                match value
                {
                    #(#match_arms)*
                    _ => Self::#fallback,
                }
            }
        }

        impl From<#enum_name> for #repr_ty
        {
            fn from(value: #enum_name) -> #repr_ty
            {
                value as #repr_ty
            }
        }
    };

    TokenStream::from(expanded)
}

pub fn derive_enum_count(input: TokenStream) -> TokenStream
{
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let data = &input.data;

    let variant_count =
        if let Data::Enum(DataEnum { variants, .. }) = data { variants.len() } else { 0 };

    let expanded = quote! {
        impl #name
        {
            #[inline]
            pub const fn count() -> usize
            {
                #variant_count
            }
        }
    };

    TokenStream::from(expanded)
}
