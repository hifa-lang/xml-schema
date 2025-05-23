use crate::xsd::{extension::Extension, xsd_context::XsdContext};
use proc_macro2::TokenStream;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespaces = {
    "xs" = "http://www.w3.org/2001/XMLSchema"
  })]
pub struct ComplexContent {
  pub extension: Option<Extension>,
}

impl ComplexContent {
  pub fn get_field_implementation(
    &self,
    namespace_definition: &TokenStream,
    context: &XsdContext,
    prefix: &Option<String>,
  ) -> TokenStream {
    self
      .extension
      .as_ref()
      .unwrap()
      .get_field_implementation(namespace_definition, context, prefix)
  }

  pub fn get_sub_type_implementation(
    &self,
    namespace_definition: &TokenStream,
    context: &XsdContext,
    prefix: &Option<String>,
  ) -> TokenStream {
    self
      .extension
      .as_ref()
      .unwrap()
      .get_sub_type_implementation(namespace_definition, context, prefix)
  }
}
