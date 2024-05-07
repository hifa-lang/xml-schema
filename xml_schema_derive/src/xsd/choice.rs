use crate::xsd::{
  annotation::Annotation, attribute::Attribute, element::Element, max_occurences::MaxOccurences,
  Implementation, XsdContext,
};
use proc_macro2::TokenStream;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
    rename = "choice"
    prefix = "xs",
    namespace = "xs: http://www.w3.org/2001/XMLSchema"
  )]
pub struct Choice {
  #[yaserde(attribute)]
  pub id: Option<String>,
  #[yaserde(rename = "attribute")]
  pub attributes: Vec<Attribute>,
  #[yaserde(rename = "minOccurs", attribute)]
  pub min_occurences: Option<u64>,
  #[yaserde(rename = "maxOccurs", attribute)]
  pub max_occurences: Option<MaxOccurences>,
  #[yaserde(rename = "annotation")]
  pub annotation: Option<Annotation>,
  #[yaserde(rename = "element")]
  pub elements: Vec<Element>,
}

impl Implementation for Choice {
  fn implement(
    &self,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
    context: &XsdContext,
  ) -> TokenStream {
    let elements: TokenStream = self
      .elements
      .iter()
      .map(|element| element.implement(&namespace_definition, prefix, context))
      .collect();

    quote! {
      #elements
    }
  }
}

impl Choice {
  pub fn get_sub_types_implementation(
    &self,
    context: &XsdContext,
    namespace_definition: &TokenStream,
    prefix: &Option<String>,
  ) -> TokenStream {
    self
      .elements
      .iter()
      .map(|element| element.get_subtypes_implementation(namespace_definition, prefix, context))
      .collect()
  }

  pub fn get_field_implementation(
    &self,
    context: &XsdContext,
    prefix: &Option<String>,
  ) -> TokenStream {
    let multiple = matches!(self.min_occurences, Some(min_occurences) if min_occurences > 1)
      || matches!(self.max_occurences, Some(MaxOccurences::Unbounded))
      || matches!(self.max_occurences, Some(MaxOccurences::Number{value}) if value > 1);

    let optional = !multiple;

    self
      .elements
      .iter()
      .map(|element| element.get_field_implementation(context, prefix, multiple, optional))
      .collect()
  }
}
