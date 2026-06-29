## 2024-05-24 - Avoid `.collect::<Vec<_>>().join("")` for string concatenation in Rust
**Learning:** Found a pattern where multiple strings are aggregated using `.collect::<Vec<_>>().join("")`, which introduces intermediate `Vec` allocations.
**Action:** Use `.collect::<String>()` directly on an iterator of strings or string slices instead, which avoids the intermediate vector allocation when joining with an empty string.

## 2024-05-24 - Use `to_lowercase().to_string()` over `to_lowercase().collect::<String>()`
**Learning:** `char::to_lowercase()` returns an iterator (since a char can map to multiple chars), and using `.collect::<String>()` on it has collection overhead.
**Action:** `char::to_lowercase()` implements `Display`, so prefer `.to_string()` over `.collect::<String>()` to leverage its Display implementation directly for string conversion.
