use crate::xsd::{
  rust_types_mapping::RustTypesMapping, sequence::Sequence, Implementation, XsdContext,
};
use heck::ToUpperCamelCase;
use proc_macro2::{Span, TokenStream};
use syn::Ident;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespaces = {
    "xs" = "http://www.w3.org/2001/XMLSchema"
  })]
pub struct Group {
  #[yaserde(attribute = true)]
  pub name: Option<String>,
  #[yaserde(attribute = true, rename = "ref")]
  pub reference: Option<String>,
  #[yaserde()]
  pub sequence: Option<Sequence>,
}

impl Implementation for Group {
  fn implement(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    if self.name.is_none() {
      return quote!();
    }
    let raw_name = self.name.clone().unwrap();

    let struct_name = Ident::new(&raw_name.to_upper_camel_case(), Span::call_site());

    let fields = self
      .sequence
      .as_ref()
      .map(|sequence| sequence.get_field_implementation(context, prefix))
      .unwrap_or_default();

    quote!(
      #[derive(Clone, Debug, Default, PartialEq, yaserde_derive::YaDeserialize, yaserde_derive::YaSerialize)]
      #namespace_definition
      pub struct #struct_name {
        #fields
      }
    )
  }
}

impl Group {
  pub fn get_type_implementation(
    &self,
    context: &XsdContext,
    _prefix: &Option<String>,
  ) -> TokenStream {
    if let Some(reference) = &self.reference {
      RustTypesMapping::get(context, reference)
    } else {
      panic!("Missing reference for group");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use hifa_yaserde::de::from_str;

  #[test]
  fn check_group_implementation() {
    let xml = r#"
        <group name="groupthing">
          <sequence>
            <element name="CX_X" type="asdfg"/>
            <element name="CY_X" type="asdfg"/>
          </sequence>
        </group>
    "#;

    let group: Group = from_str(xml).unwrap();

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let implementation = format!("{}", group.implement(&TokenStream::new(), &None, &context));

    assert_eq!(implementation, "# [derive (Clone , Debug , Default , PartialEq , serde :: Deserialize , serde :: Serialize)] \
pub struct Groupthing { \
# [yaserde (rename = \"CX_X\")] pub cx_x : xml_schema_types :: Asdfg , \
# [yaserde (rename = \"CY_X\")] pub cy_x : xml_schema_types :: Asdfg , }");
  }

  #[test]
  fn check_group_ref() {
    let xml = r#"<group ref="bla:groupthing" />"#;

    let group: Group = from_str(xml).unwrap();

    let context =
      XsdContext::new(r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"></xs:schema>"#)
        .unwrap();

    let type_implementation = format!("{}", group.get_type_implementation(&context, &None));

    assert_eq!(type_implementation, "Groupthing");
  }
}
