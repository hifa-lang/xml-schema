use crate::xsd::{
  rust_types_mapping::RustTypesMapping, simple_type::SimpleType, Implementation, XsdContext,
};
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Span, TokenStream};
use syn::Ident;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "attribute",
  prefix = "xs",
  namespaces = {
    "xs" = "http://www.w3.org/2001/XMLSchema"
  }
)]
pub struct Attribute {
  #[yaserde(prefix = "xs", attribute = true)]
  pub name: Option<String>,
  #[yaserde(rename = "type", attribute = true)]
  pub kind: Option<String>,
  //#[yaserde(attribute = true)]
  // pub default: Option<String>,
  //#[yaserde(attribute = true)]
  // pub fixed: Option<String>,
  #[yaserde(rename = "use", attribute = true)]
  pub required: Required,
  #[yaserde(rename = "ref", attribute = true)]
  pub reference: Option<String>,
  #[yaserde(rename = "simpleType")]
  pub simple_type: Option<SimpleType>,
}

#[derive(Clone, Debug, Default, PartialEq, YaDeserialize)]
pub enum Required {
  #[default]
  #[yaserde(rename = "optional")]
  Optional,
  #[yaserde(rename = "required")]
  Required,
}

impl Implementation for Attribute {
  fn implement(
    &self,
    _namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    if self.name.is_none() {
      return quote!();
    }

    let raw_name = self.get_raw_name().unwrap();
    let name = self.get_name().unwrap();

    let field_name = Ident::new(&name, Span::call_site());

    let rust_type = match (
      self.reference.as_ref(),
      self.kind.as_ref(),
      self.simple_type.as_ref(),
    ) {
      (None, Some(kind), None) => RustTypesMapping::get(context, kind),
      (Some(reference), None, None) => RustTypesMapping::get(context, reference),
      (None, None, Some(simple_type)) => {
        let struct_name: Option<Ident> = if let Some(name) = self.name.as_ref() {
          Some(Ident::new(&name.to_upper_camel_case(), Span::call_site()))
        } else {
          None
        };

        simple_type.get_type_implementation(context, prefix, struct_name.as_ref())
      }
      (_, _, _) => panic!("Not implemented Rust type for: {:?}", self),
    };

    let required = match self.simple_type.as_ref() {
      Some(SimpleType { list: Some(_), .. }) => true,
      _ => false,
    };

    let rust_type = if !required && self.required == Required::Optional {
      quote!(Option<#rust_type>)
    } else {
      quote!(#rust_type)
    };

    let attributes = if name == raw_name {
      quote!(attribute = true)
    } else {
      quote!(attribute = true, rename=#raw_name)
    };

    // TODO: add support for default and fixed attributes
    // let prefix_attribute = prefix
    //   .as_ref()
    //   .map(|prefix| quote!(, prefix=#prefix))
    //   .unwrap_or_default();

    quote!(
      #[yaserde(#attributes)]
      pub #field_name: #rust_type,
    )
  }
}

impl Attribute {
  pub fn get_sub_type_implementation(
    &self,
    namespace_definition: &TokenStream,
    context: &XsdContext,
    prefix: &Option<String>,
  ) -> TokenStream {
    let simple_types: TokenStream = {
      let mut context = context.clone();
      context.set_is_in_sub_module(true);

      self
        .simple_type
        .iter()
        .map(|simple_type| {
          if simple_type.name.is_empty() {
            let mut simple_type = simple_type.clone();
            simple_type.name = self.get_name().unwrap_or_default();
            simple_type.implement(&namespace_definition, prefix, &context)
          } else {
            simple_type.implement(&namespace_definition, prefix, &context)
          }
        })
        .collect()
    };

    quote! {
      #simple_types
    }
  }

  fn get_name(&self) -> Option<String> {
    if let Some(raw_name) = self.name.as_ref() {
      let name = raw_name.to_snake_case();

      if name == "type" {
        Some("kind".to_string())
      } else {
        Some(name)
      }
    } else {
      None
    }
  }

  fn get_raw_name(&self) -> Option<String> {
    if let Some(name) = &self.name {
      Some(name.clone())
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  #[test]
  fn default_required() {
    assert_eq!(Required::default(), Required::Optional);
  }

  #[test]
  fn string_attribute() {
    let attribute = Attribute {
      name: Some("language".to_string()),
      kind: Some("xs:string".to_string()),
      reference: None,
      required: Required::Required,
      simple_type: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = attribute.implement(&TokenStream::new(), &None, &context);

    let expected = TokenStream::from_str(
      r#"
       #[yaserde(attribute = true)]
        pub language: String,
      "#,
    )
    .unwrap();

    assert_eq!(implementation.to_string(), expected.to_string());
  }

  #[test]
  fn optional_string_attribute() {
    let attribute = Attribute {
      name: Some("language".to_string()),
      kind: Some("xs:string".to_string()),
      reference: None,
      required: Required::Optional,
      simple_type: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = attribute.implement(&TokenStream::new(), &None, &context);

    let expected = TokenStream::from_str(
      r#"
       #[yaserde(attribute = true)]
        pub language: Option<String> ,
      "#,
    )
    .unwrap();

    assert_eq!(implementation.to_string(), expected.to_string());
  }

  #[test]
  fn type_attribute() {
    let attribute = Attribute {
      name: Some("type".to_string()),
      kind: Some("xs:string".to_string()),
      reference: None,
      required: Required::Optional,
      simple_type: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = attribute.implement(&TokenStream::new(), &None, &context);

    let expected = TokenStream::from_str(
      r#"
        #[yaserde(attribute = true, rename="type")]
        pub kind: Option<String> ,
      "#,
    )
    .unwrap();

    assert_eq!(implementation.to_string(), expected.to_string());
  }

  #[test]
  fn reference_type_attribute() {
    let attribute = Attribute {
      name: Some("type".to_string()),
      kind: None,
      reference: Some("MyType".to_string()),
      required: Required::Optional,
      simple_type: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = attribute.implement(&TokenStream::new(), &None, &context);

    let expected = TokenStream::from_str(
      r#"
        #[yaserde(attribute = true, rename="type")]
        pub kind: Option<MyType> ,
      "#,
    )
    .unwrap();

    assert_eq!(implementation.to_string(), expected.to_string());
  }

  #[test]
  #[should_panic]
  fn bad_type_attribute() {
    let attribute = Attribute {
      name: Some("type".to_string()),
      kind: None,
      reference: None,
      required: Required::Optional,
      simple_type: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    attribute.implement(&TokenStream::new(), &None, &context);
  }

  #[test]
  fn attribute_without_name() {
    let attribute = Attribute {
      name: None,
      kind: Some("xs:string".to_string()),
      reference: None,
      required: Required::Optional,
      simple_type: None,
    };

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = attribute
      .implement(&TokenStream::new(), &None, &context)
      .to_string();
    assert!(implementation.is_empty());
  }
}
