## 2024-05-18 - [Rust Serde Deserialization Memory Optimization]
**Learning:** In Rust's Serde, deep cloning `Value` structures (from `serde_json` or `serde_yaml`) just to pass to a deserializer (`T::deserialize(val)`) causes a significant number of expensive intermediate allocations.
**Action:** Always prefer borrowing from the parsed `Value` directly via `&val` to map to the structure if `from_value` requires an owned type and you are not consuming the tree, thus avoiding the deep clone tax.
