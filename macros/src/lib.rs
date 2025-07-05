use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Attribute, DeriveInput, Expr, Lit, Meta, MetaNameValue, parse_macro_input};

/// Adds a constant `MESSAGE_ID: u8` to a struct or enum and impl. an `encode` method that calls
/// `self.encode_content() -> Vec<u8>` and prepends the `MESSAGE_ID` to the byte vector
#[proc_macro_derive(MessageId, attributes(message_id))]
pub fn message_id(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match message_id_impl(input) {
        Ok(stream) => stream,
        Err(error) => error.into_compile_error(),
    }
    .into()
}

fn message_id_impl(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let ident = input.ident;

    let message_id = MessageId::parse(&input.attrs)?.0;

    let expanded = quote! {
        impl #ident {
            const MESSAGE_ID: u8 = #message_id;

            pub fn encode(self) -> Vec<u8> {
                let mut bytes = self.encode_content();

                bytes.insert(0, Self::MESSAGE_ID);

                bytes
            }
        }
    };

    Ok(expanded)
}

struct MessageId(u8);

impl MessageId {
    /// Parses #[message_id = <u8>] attribute into `MessageId`
    fn parse(input: &[Attribute]) -> syn::Result<MessageId> {
        for attribute in input {
            let Meta::NameValue(MetaNameValue { path, value, .. }) = &attribute.meta else {
                continue;
            };

            if !path.is_ident("message_id") {
                continue;
            }

            if let Expr::Lit(expr_lit) = value {
                if let Lit::Int(ref lit_int) = expr_lit.lit {
                    let id = lit_int.base10_parse::<u8>()?;
                    return Ok(MessageId(id));
                }
            }
        }

        Err(syn::Error::new(
            Span::call_site(),
            "You need to specify the message id with: #[message_id = <u8>]",
        ))
    }
}
