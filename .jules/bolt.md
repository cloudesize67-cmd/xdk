## 2024-05-24 - Avoid string collection allocations
**Learning:** Rust's `.collect::<Vec<_>>().join("")` allocates an unnecessary intermediate vector, and `.to_lowercase().collect::<String>()` misses out on `Display` optimizations compared to `.to_lowercase().to_string()`.
**Action:** Use `.collect::<String>()` directly when concatenating strings from an iterator, and `.to_string()` for char transformations to save memory and CPU cycles.
