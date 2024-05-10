use crate::xsd::{enumeration::Enumeration, rust_types_mapping::RustTypesMapping, XsdContext};
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use super::Implementation;

#[derive(Clone, Default, Debug, PartialEq, YaDeserialize)]
#[yaserde(prefix = "xs", namespace = "xs: http://www.w3.org/2001/XMLSchema")]
pub struct Restriction {
  #[yaserde(rename = "base", attribute)]
  pub base: Option<String>,
  #[yaserde(rename = "enumeration")]
  pub enumerations: Vec<Enumeration>,
}

impl Implementation for Restriction {
  fn implement_childs(
    &self,
    namespace_definition: &TokenStream,
    _prefix: &Option<String>,
    context: &XsdContext,
    struct_name: &syn::Ident,
  ) -> TokenStream {
    if let Some(base) = self.base.as_ref() {
      if RustTypesMapping::is_xs_string(context, base) {
        if self.enumerations.len() > 0 {
          let enum_values_string: Vec<String> = self
            .enumerations
            .iter()
            .map(|enumeration| enumeration.value.clone())
            .collect::<Vec<String>>();
          let enum_values: Vec<Ident> = enum_values_string
            .iter()
            .map(|enumeration| Ident::new(enumeration, Span::call_site()))
            .collect::<Vec<Ident>>();

          return quote!(
            #[derive(Clone, Debug, Default, PartialEq, hifa_yaserde_derive::YaDeserialize, hifa_yaserde_derive::YaSerialize)]
            pub enum #struct_name {
              #[default]
              _DEFAULT,
              #(#enum_values),*

            }

            impl std::str::FromStr for #struct_name {
              type Err = String;

              fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                  #(#enum_values_string => Ok(#struct_name::#enum_values),)*
                  _ => Err(format!("Invalid value: {}", s)),
                }
              }
            }

            impl std::string::ToString for #struct_name {
              fn to_string(&self) -> String {
                match self {
                  #struct_name::_DEFAULT => panic!("Cannot convert _DEFAULT to string"),
                  #(#struct_name::#enum_values => #enum_values_string.to_string(),)*
                }
              }
            }
          );
        } else {
          return quote!(
            #[derive(Clone, Debug, Default, PartialEq, hifa_yaserde_derive::YaDeserialize, hifa_yaserde_derive::YaSerialize)]
            #namespace_definition
            pub struct #struct_name {
              #[yaserde(text)]
              pub content: std::string::String,
            }
          );
        }
      }
    }

    unimplemented!("unimplemented Restriction: {:?}", self);
  }
}

impl Restriction {
  pub fn get_type_implementation(
    &self,
    context: &XsdContext,
    _prefix: &Option<String>,
  ) -> TokenStream {
    if let Some(base) = &self.base {
      RustTypesMapping::get(context, base)
    } else {
      panic!("Missing base for restriction");
    }
  }

  // quote!(
  //   #[derive(Clone, Debug, Default, PartialEq, hifa_yaserde_derive::YaDeserialize, hifa_yaserde_derive::YaSerialize)]
  //   #namespace_definition
  //   pub struct #struct_name {
  //     #[yaserde(text)]
  //     pub content: std::string::String,
  //   }
  // )

  //   fn implement_string_enum(
  //     &self,
  //     _namespace_definition: &TokenStream,
  //     _prefix: &Option<String>,
  //     context: &XsdContext,
  //     struct_name: &str,
  //   ) -> TokenStream {
  //     let list_type = RustTypesMapping::get(context, &self.item_type);

  //     quote!(
  //       #[derive(Clone, Debug, Default, PartialEq)]
  //       pub struct #struct_name {
  //         pub items: Vec<#list_type>
  //       }

  //       impl hifa_yaserde::YaDeserialize for #struct_name {
  //         fn deserialize<R: std::io::Read>(reader: &mut hifa_yaserde::de::Deserializer<R>) -> Result<Self, String> {
  //           loop {
  //             match reader.next_event()? {
  //               xml::reader::XmlEvent::StartElement{..} => {}
  //               xml::reader::XmlEvent::Characters(ref text_content) => {
  //                 let items: Vec<#list_type> =
  //                   text_content
  //                     .split(' ')
  //                     .map(|item| item.to_owned())
  //                     .map(|item| item.parse().unwrap())
  //                     .collect();

  //                 return Ok(#struct_name {items});
  //               }
  //               _ => {break;}
  //             }
  //           }

  //           Err("Unable to parse attribute".to_string())
  //         }
  //       }

  //       impl hifa_yaserde::YaSerialize for #struct_name {
  //         fn serialize<W: std::io::Write>(&self, writer: &mut hifa_yaserde::ser::Serializer<W>) -> Result<(), String> {
  //           let content =
  //             self.items.iter().map(|item| item.to_string()).collect::<Vec<String>>().join(" ");

  //           let data_event = xml::writer::XmlEvent::characters(&content);
  //           writer.write(data_event).map_err(|e| e.to_string())?;

  //           Ok(())
  //         }

  //         fn serialize_attributes(&self, mut source_attributes: Vec<xml::attribute::OwnedAttribute>, mut source_namespace: xml::namespace::Namespace) -> Result<(Vec<xml::attribute::OwnedAttribute>, xml::namespace::Namespace), String> {
  //           Ok((source_attributes, source_namespace))
  //         }
  //       }
  //     )
  //   }
}
