use hifa_yaserde::de::from_str;
use hifa_yaserde::ser::to_string;
use xml_schema_derive::XmlSchema;

#[test]
fn complex_type_string() {
  #[derive(Debug, XmlSchema)]
  #[xml_schema(source = "xml_schema/tests/complex_type.xsd")]
  struct ComplexTypeSchema;

  let xml_1 = r#"
  <ComplexListOfElements>
    <Annotation>Test content</Annotation>
    <Label>Label content</Label>
  </ComplexListOfElements>
  "#;

  let sample_1: xml_schema_types::ComplexListOfElements = from_str(xml_1).unwrap();

  let model = xml_schema_types::ComplexListOfElements {
    annotation: Some("Test content".to_string()),
    label: "Label content".to_string(),
  };

  assert_eq!(sample_1, model);

  let data = to_string(&model).unwrap();
  assert_eq!(
    data,
    r#"<?xml version="1.0" encoding="utf-8"?><ComplexListOfElements><Annotation>Test content</Annotation><Label>Label content</Label></ComplexListOfElements>"#
  );
}
