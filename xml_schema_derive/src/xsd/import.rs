#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(
  rename = "schema",
  prefix = "xs",
  namespaces = {
    "xs" = "http://www.w3.org/2001/XMLSchema"
  }
)]
pub struct Import {
  #[yaserde(attribute = true)]
  pub id: Option<String>,
  #[yaserde(attribute = true)]
  pub namespace: Option<String>,
  #[yaserde(rename = "schemaLocation", attribute = true)]
  pub schema_location: Option<String>,
}
