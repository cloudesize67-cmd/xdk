## 2024-05-18 - Performance Optimization: avoid deep clones when deserializing
**Learning:** In xdk-openapi, the components parser was using `from_value(value.clone())` extensively. When a `Value` struct represents an object like a Schema or Parameter, deep cloning it is expensive. `T::deserialize(&value)` can be used instead because `&Value` implements the `Deserializer` trait, preventing a full clone of the data structure.
**Action:** Always prefer `T::deserialize(&value)` over `from_value(value.clone())` when parsing intermediate JSON/YAML nodes in Rust.
