use darling::Error;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod metadata;

#[proc_macro_attribute]
pub fn metadata(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);

    metadata::new(attr.into(), item)
        .unwrap_or_else(|err| TokenStream::from(Error::from(err).write_errors()).into())
        .into()
}
