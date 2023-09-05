![ci](https://github.com/jsturtevant/hcn-rs/actions/workflows/ci.yml/badge.svg)

# hcn-rs

Library for working with the [Windows Host Compute Network](https://learn.microsoft.com/en-us/windows-server/networking/technologies/hcn/hcn-top).

## Example

```
use hcn::get_namespace;

let namespace = get_namespace(api_namespace.id.as_str())?;
println!("Namespace id: {}", namespace.id);
```

See [examples folder](examples) for more.

## JSON schema 

The [HCN API Schema](https://learn.microsoft.com/en-us/virtualization/api/hcn/hns_schema) is exposed as a module that can be used to call the API. 

## Low Level API

The library also has a low level API that translates the HCN C library to Rust friendly implementation. This is used throughout the project and can provide flexibility if the schema hasn't been updated yet but does require additional steps.  See the `*_api.rs` in the [examples folder](examples)

### Credit
The low level api was originally from https://github.com/rafawo/hcs-rs under MIT.  This project updated the API's to use https://github.com/microsoft/windows-rs, updated error handling, changed handle types, added HCN schema and wrappers around the API to simplify its use.
