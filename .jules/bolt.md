## 2024-05-19 - [Avoid deep clones when parsing OpenAPI specifications]
**Learning:** OpenAPI specifications can be massive. Deep cloning `serde_json::Value` or `serde_yaml::Value` nodes during parsing (using `.clone()` with `from_value`) adds massive, unnecessary overhead.
**Action:** When extracting data from intermediate DOM nodes using `serde`, prefer deserializing directly from the borrowed references (`&Value`) using `T::deserialize(value)` instead of `T::from_value(value.clone())`. This avoids deep clones and drastically reduces allocations.
