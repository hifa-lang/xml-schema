use hifa_xml_schema_derive::XmlSchema;
use hifa_yaserde::de::from_str;
use hifa_yaserde::ext::Boxed;
use hifa_yaserde::ser::to_string;

#[test]
fn complex_type_string() {
  #[derive(Debug, XmlSchema)]
  #[xml_schema(source = "tests/complex_type.xsd")]
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
    r#"<?xml version="1.0" encoding="UTF-8"?><ComplexListOfElements><Annotation>Test content</Annotation><Label>Label content</Label></ComplexListOfElements>"#
  );
}

#[test]
fn complex_type_sequence_choice() {
  #[derive(Debug, XmlSchema)]
  #[xml_schema(source = "tests/complex_type_sequence_choice.xsd")]
  struct ComplexTypeSchema;

  let xml_1 = r#"
  <Animals>
    <Cat name="AAA" />
    <Dog name="BBB" />
    <Cat name="CCC" />
  </Animals>
  "#;

  let sample_1: xml_schema_types::Animals = from_str(xml_1).unwrap();

  let model = xml_schema_types::Animals {
    cat_list: vec![
      Boxed::from(xml_schema_types::Cat {
        name: "AAA".to_string(),
      }),
      Boxed::from(xml_schema_types::Cat {
        name: "CCC".to_string(),
      }),
    ],
    dog_list: vec![Boxed::from(xml_schema_types::Dog {
      name: "BBB".to_string(),
    })],
  };

  assert_eq!(sample_1, model);

  let data = to_string(&model).unwrap();
  assert_eq!(
    data,
    r#"<?xml version="1.0" encoding="UTF-8"?><Animals><Cat name="AAA" /><Cat name="CCC" /><Dog name="BBB" /></Animals>"#
  );
}
