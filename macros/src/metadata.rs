use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::DeriveInput;

pub(crate) fn new(attr: TokenStream, input: DeriveInput) -> Result<TokenStream, Error> {
    let attr = match NestedMeta::parse_meta_list(attr.into()) {
        Ok(v) => v,
        Err(err) => return Err(Error::from(err)),
    };
    Ok(MetadataImpl::new(&attr, &input)?.into_token_stream())
}

struct MetadataImpl<'a> {
    id: String,
    description: String,
    page: String,
    input: &'a DeriveInput,
}

impl<'a> MetadataImpl<'a> {
    fn new(attr: &[NestedMeta], input: &'a DeriveInput) -> Result<Self, Error> {
        #[derive(FromMeta)]
        struct MacroArgs {
            #[darling(default)]
            id: String,
            #[darling(default)]
            description: String,
            #[darling(default)]
            page: String,
        }
        let args = MacroArgs::from_list(&attr)?;

        Ok(Self {
            id: args.id,
            description: args.description,
            page: args.page,
            input: &input,
        })
    }
}

impl<'a> ToTokens for MetadataImpl<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            id,
            description,
            page,
            input,
            ..
        } = self;

        let struct_name = &input.ident;

        tokens.extend(quote! {
          #input

          impl types::Metadata for #struct_name {
            fn id(&self) -> String {
              #id.to_string()
            }
            fn description(&self) -> String {
              #description.to_string()
            }
            fn page(&self) -> String {
              #page.to_string()
            }
          }
        });
    }
}
