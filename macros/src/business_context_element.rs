use crate::utils::add_field;
use proc_macro2::TokenStream;
use syn::fold::Fold;
use syn::{parse2, FieldsNamed, ItemStruct};

struct Processor;

impl Fold for Processor {
  fn fold_fields_named(&mut self, mut fields: FieldsNamed) -> FieldsNamed {
    add_field(
      &mut fields,
      quote! {
        /// The URI of this business context element.
        pub(crate) uri: Option<String>
      },
    );
    fields
  }
}

pub(crate) fn process(item: TokenStream) -> TokenStream {
  fn process_item(item: TokenStream) -> syn::Result<TokenStream> {
    let input: ItemStruct = parse2(item)?;
    let input = Processor.fold_item_struct(input);
    let name = &input.ident;
    let expanded = quote! {
      impl BusinessContextElement for #name {
        /// Returns the URI of this business context element.
        fn uri(&self) -> &Option<String> {
          &self.uri
        }
      }
    };
    Ok(quote! {
      #input
      #expanded
    })
  }

  process_item(item).unwrap_or_else(syn::Error::into_compile_error)
}
