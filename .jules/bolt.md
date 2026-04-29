## 2023-10-27 - [Avoid String concatenation allocation in casing generation]
**Learning:** In Rust, avoid allocating a `Vec` for string concatenation (e.g. `.collect::<Vec<_>>().join("")`). The `FromIterator` implementation on `String` can collect an iterator of characters or string slices directly into a `String` without the intermediate vector allocation.
**Action:** Replace `words.iter().map(|w| pascal_case(w)).collect::<Vec<_>>().join("")` with `words.iter().map(|w| pascal_case(w)).collect::<String>()` to eliminate the unnecessary `Vec` allocation.

## 2023-10-27 - [Avoid Vec allocations in Iterator]
**Learning:** Avoid allocating a `Vec` when taking a fixed number of elements from an iterator.
**Action:** Replace `let parts = line.split(" ").collect::<Vec<&str>>();` with `.next()` calls on the iterator, avoiding unnecessary allocations.
