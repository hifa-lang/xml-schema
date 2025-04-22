#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespaces = {
    "xs" = "http://www.w3.org/2001/XMLSchema"
  })]
pub struct Enumeration {
 #[yaserde(attribute = true)]
  pub value: String,
}
