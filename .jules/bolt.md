## 2024-05-18 - Optimize Serde Value Deserialization
**Learning:** Deserializing from intermediate `serde_json::Value` or `serde_yaml::Value` using `T::deserialize(&value)` is faster than `from_value(value.clone())` as it avoids expensive deep clones of the value tree. Since `&Value` implements `Deserializer`, this reduces heap allocations and is particularly beneficial for large specifications.
**Action:** Use `T::deserialize(&value)` instead of `from_value(value.clone())` for intermediate Serde JSON/YAML values.
