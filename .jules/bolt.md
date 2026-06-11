## 2024-05-18 - Avoid Vector Allocation in Pascal Case Join

**Learning:** When generating Pascal case from a slice of strings in Rust, mapping to strings and then collecting into a `Vec` before calling `join("")` causes an intermediate vector allocation that is entirely unnecessary.
**Action:** Use `.map(|w| pascal_case(w)).collect::<String>()` instead. The `FromIterator` implementation for `String` handles the allocation and joining directly, which is faster and avoids the temporary `Vec`.

## 2024-05-18 - String Conversion from Char Iterator

**Learning:** When lowercasing or uppercasing a single character in Rust using `.to_lowercase()`, it returns an iterator. Collecting this iterator into a `String` using `.collect::<String>()` is slower than using `.to_string()`, because `to_string()` can utilize the existing `Display` implementation for the iterator instead of allocating and populating from an iterator loop.
**Action:** Always prefer `.to_string()` over `.collect::<String>()` for `char::ToLowercase` and `char::ToUppercase` iterators to avoid iterator overhead and use optimized formatting.
