## 2024-05-18 - [Avoid intermediate allocations when concatenating string]
**Learning:** In Rust, `.collect::<Vec<_>>().join("")` creates an intermediate `Vec` when joining strings. We can use `.collect::<String>()` instead to allocate a single String directly from the iterator, avoiding unnecessary allocations. This is highly beneficial in string manipulation hot paths.
**Action:** When transforming and joining strings in Rust, avoid `collect::<Vec<_>>().join("")` and prefer `.collect::<String>()` for direct concatenation.
