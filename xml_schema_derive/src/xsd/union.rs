#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespaces = {
    "xs" = "http://www.w3.org/2001/XMLSchema"
  })]
pub struct Union {
  #[yaserde(rename = "memberTypes", attribute = true)]
  pub member_types: String,
}
