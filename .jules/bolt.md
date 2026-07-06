## 2024-07-06 - [Avoid intermediate allocations in string transformations]
**Learning:** In Rust, when transforming and concatenating a collection of strings, calling `.collect::<Vec<_>>().join("")` allocates an intermediate `Vec` which is unnecessary. Directly collecting into a `String` is much faster as it avoids creating a `Vec` entirely.
**Action:** Use `.collect::<String>()` instead of `.collect::<Vec<_>>().join("")` for concatenating string iterators.
