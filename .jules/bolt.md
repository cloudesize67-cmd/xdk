## 2024-05-18 - [Avoid deep cloning Value trees for deserialization]
**Learning:** Using `T::deserialize(&value)` on `serde_json::Value` or `serde_yaml::Value` utilizes the `Deserializer` implementation on `&Value`, avoiding expensive deep clones compared to using `serde_json::from_value(value.clone())`.
**Action:** Always prefer `T::deserialize(&value)` over `from_value(value.clone())` when deserializing from dynamically typed intermediate representations.
