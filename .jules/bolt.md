## 2024-05-18 - Optimize string allocations in Casing conversions
**Learning:** In Rust, avoid `.collect::<Vec<_>>().join("")` when you just want to concatenate strings. Collecting into a `Vec` allocates unnecessary intermediate memory.
**Action:** Use `.collect::<String>()` directly on an iterator that yields string-like objects to prevent unnecessary `Vec` allocations.
