# hifa xml-schema

[![Latest Version]][crates.io]

[Latest Version]: https://img.shields.io/crates/v/hifa-xml-schema.svg
[crates.io]: https://crates.io/crates/hifa-xml-schema

Generate rust code (structures and enum) from XSD

This is a modified version of the project in [media-io/xml-schema](https://github.com/media-io/xml-schema).

## Requirements

This project depends to other required libraries to start with, add them to your `Cargo.toml`.

- [hifa YaSerDe derive](https://crates.io/crates/hifa_yaserde_derive)
- [hifa YaSerDe](https://crates.io/crates/hifa_yaserde)
- [hifa XML-Schema derive](https://crates.io/crates/hifa-xml-schema-derive)
- [hifa XML-Schema](https://crates.io/crates/hifa-xml-schema)
- [XML-rs](https://crates.io/crates/xml-rs)
- [log](https://crates.io/crates/log)

## Usage

In the entrypoint of your rust project, add these folowing lines:

```rust
#[macro_use]
extern crate hifa_yaserde_derive;

use std::io::prelude::*;
use hifa_xml_schema_derive::XmlSchema;
use hifa_yaserde::{YaDeserialize, YaSerialize};
```

Then implement the XSD using:

```rust
#[derive(Debug, XmlSchema)]
#[xml_schema(source = "path_to_schema.xsd", target_prefix = "my_prefix")]
struct MySchema;
```

Remark: the `MySchema` don't need to be public. It serve just as support of information.

### Attributes

**source**: Source of the XSD - XML Schema. It can be local file (related to the root of the project) or an HTTP resource.  
**target_prefix**: The schema not define any prefix. It the `targetNamespace` is declared in the schema, this attribute is required.  
**store_generated_code**: Optional attribute for debug purpose. It store the generated Rust code into the file - the attribute value is the output filename.  
**log_level**: To configure the logger level at the the compile time - usefull if the XSD generate some bugs. Values can be `error`, `warn`, `info`, `debug`, `trace`.  
**module_namespace_mapping**: map a namespace to a Rust module. It can be present many times to map multiple namespaces to different Rust modules.
