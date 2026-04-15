## 2024-05-23 - Avoid Expensive Deserialization Clones
**Learning:** Deserializing from intermediate `serde_json::Value` or `serde_yaml::Value` using `from_value(value.clone())` causes expensive deep cloning of the entire value tree.
**Action:** Prefer `T::deserialize(&value)` which avoids deep clones and heap allocations, especially important for large specifications, as `&Value` implements `Deserializer`.
