use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Error, Field, Result, spanned::Spanned};

const SPAN_ATTRIBUTE: &str = "span";

pub fn expand(input: DeriveInput) -> Result<TokenStream> {
    match &input.data {
        Data::Struct(data_struct) => expand_struct(&input, data_struct),
        Data::Enum(_) => Err(Error::new(input.span(), "enums are not supported")),
        Data::Union(_) => Err(Error::new(input.span(), "unions are not supported")),
    }
}

fn expand_struct(input: &DeriveInput, data_struct: &DataStruct) -> Result<TokenStream> {
    let span_fields = data_struct
        .fields
        .iter()
        .filter(|field| is_span_field(field))
        .collect::<Vec<_>>();

    if span_fields.len() > 1 {
        return Err(Error::new_spanned(
            input,
            "struct can have only one field marked with #[span]",
        ));
    }

    let Field {
        ident: span_field, ..
    } = span_fields.first().ok_or_else(|| {
        Error::new_spanned(input, "struct must have one field marked with #[span]")
    })?;

    let struct_name = &input.ident;

    Ok(quote! {
        impl unnamed_common::Spanned for #struct_name {
            fn span(&self) -> unnamed_common::Span {
                self.#span_field
            }
        }
    })
}

fn is_span_field(field: &Field) -> bool {
    field
        .attrs
        .iter()
        .any(|attr| attr.path().is_ident(SPAN_ATTRIBUTE))
}
