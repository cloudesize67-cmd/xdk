## 2024-05-30 - Optimize string concatenation

**Learning:** Rust string iterators using `.collect::<Vec<_>>().join("")` allocate an unnecessary intermediate vector.

**Action:** Replace this pattern with `.collect::<String>()` to bypass the intermediate vector allocation.
