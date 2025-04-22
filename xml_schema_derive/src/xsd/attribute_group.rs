use crate::xsd::attribute::Attribute;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "attributeGroup",
  prefix = "xs",
  namespaces = {
    "xs" = "http://www.w3.org/2001/XMLSchema"
  }
)]
pub struct AttributeGroup {
  #[yaserde(prefix = "xs", attribute = true)]
  pub name: Option<String>,
  #[yaserde(rename = "ref", attribute = true)]
  pub reference: String,
  #[yaserde(rename = "attribute")]
  pub attributes: Vec<Attribute>,
  // #[yaserde(rename = "attributeGroup")]
  // pub attribute_group: Vec<AttributeGroup>,
}
