
## $(date +%Y-%m-%d) - Deserializing Reference Types with Serde
**Learning:** When parsing intermediate Abstract Syntax Trees (like `serde_json::Value` or `serde_yaml::Value`) to strongly-typed structs in Serde, calling `.clone()` before `from_value` forces an expensive deep copy of the entire AST subtree. `&Value` implements `Deserializer`, so you can completely bypass cloning by using `T::deserialize(&value)`.
**Action:** When working with `serde_json` and `serde_yaml` ASTs, always use `T::deserialize(&val)` instead of `serde_json::from_value(val.clone())` to avoid unnecessary memory allocation and traversal, unless you are consuming the final tree directly.
