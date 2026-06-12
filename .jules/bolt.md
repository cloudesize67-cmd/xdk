## 2024-05-24 - Serde Deserialization Overhead
**Learning:** `serde_json::from_value` and `serde_yaml::from_value` take ownership of the `Value`. Calling them with `value.clone()` results in an expensive deep clone of the entire JSON/YAML AST tree (which can be huge for OpenAPI specs).
**Action:** Always prefer `T::deserialize(&value)` which directly delegates to the reference's `Deserializer` implementation and avoids deep cloning. Only use `from_value(value)` if you are consuming the very root `Value` object and no longer need it.
