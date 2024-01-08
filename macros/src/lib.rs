//! # Derive macros

extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::parse::Parser;

#[proc_macro_derive(ToErrorMessage)]
pub fn to_error_message(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  let name = &input.ident;
  let expanded = quote! {
    impl ToErrorMessage for #name {
      fn message(self) -> String {
        self.0
      }
    }
  };
  TokenStream::from(expanded)
}

#[proc_macro_derive(DmnElement)]
pub fn dmn_element(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  let name = &input.ident;
  let expanded = quote! {
    impl DmnElement for #name {
      fn namespace(&self) -> &str {
        &self.namespace
      }
      fn model_name(&self) -> &str {
        &self.model_name
      }
      fn id(&self) -> &String {
        match &self.id {
          DmnId::Provided(id) => id,
          DmnId::Generated(id) => id,
        }
      }
      fn opt_id(&self) -> Option<&String> {
        match &self.id {
          DmnId::Provided(id) => Some(&id),
          DmnId::Generated(_) => None,
        }
      }
      fn description(&self) -> &Option<String> {
        &self.description
      }
      fn label(&self) -> &Option<String> {
        &self.label
      }
      fn extension_elements(&self) -> &Vec<ExtensionElement> {
        &self.extension_elements
      }
      fn extension_attributes(&self) -> &Vec<ExtensionAttribute> {
        &self.extension_attributes
      }
    }
  };
  TokenStream::from(expanded)
}

/*
/// Extends annotated struct with fields and methods required by DMN element.
#[proc_macro_attribute]
pub fn dmn_elements(_input: TokenStream, annotated_item: TokenStream) -> TokenStream {
  let mut item = syn::parse_macro_input!(annotated_item as syn::DeriveInput);
  let name = &item.ident;
  if let syn::Data::Struct(ref mut data_struct) = item.data {
    if let syn::Fields::Named(ref mut fields) = data_struct.fields {
      // let tokens = quote! {
      //   /// Optional type definition.
      //   pub (crate) type_ref: Option<String>
      // };
      // let field: syn::Field = syn::Field::parse_named.parse2(tokens).unwrap();
      // fields.named.push(field);
      let expanded = quote! {
        #item
        // impl Expression for #name {
        //   /// Returns a reference to optional type definition.
        //   fn type_ref(&self) -> &Option<String> {
        //     &self.type_ref
        //   }
        // }
      };
      return TokenStream::from(expanded);
    }
  }
  panic!("Attribute macro 'dmn_element' is valid only for structs");
}
*/

#[proc_macro_derive(NamedElement)]
pub fn named_element(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  let name = &input.ident;
  let expanded = quote! {
    impl NamedElement for #name {
      fn name(&self) -> &str {
        &self.name
      }
      fn feel_name(&self) -> &Name {
        &self.feel_name
      }
    }
  };
  TokenStream::from(expanded)
}

#[proc_macro_derive(BusinessContextElement)]
pub fn business_context_element(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);
  let name = &input.ident;
  let expanded = quote! {
    impl BusinessContextElement for #name {
      fn uri(&self) -> &Option<String> {
        &self.uri
      }
    }
  };
  TokenStream::from(expanded)
}

/// Extends annotated struct with fields and methods required by DMN expressions.
#[proc_macro_attribute]
pub fn expression(_input: TokenStream, annotated_item: TokenStream) -> TokenStream {
  let mut item = syn::parse_macro_input!(annotated_item as syn::DeriveInput);
  let name = &item.ident;
  if let syn::Data::Struct(ref mut data_struct) = item.data {
    if let syn::Fields::Named(ref mut fields) = data_struct.fields {
      let tokens = quote! {
        /// Optional type definition.
        pub (crate) type_ref: Option<String>
      };
      let field: syn::Field = syn::Field::parse_named.parse2(tokens).unwrap();
      fields.named.push(field);
      let expanded = quote! {
        #item
        impl Expression for #name {
          /// Returns a reference to optional type definition.
          fn type_ref(&self) -> &Option<String> {
            &self.type_ref
          }
        }
      };
      return TokenStream::from(expanded);
    }
  }
  panic!("Attribute macro 'expression' is valid only for structs");
}
