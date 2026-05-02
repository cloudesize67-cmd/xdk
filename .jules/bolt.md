## 2024-05-23 - [Optimization: Avoid Vec Allocations in Iterators]
**Learning:** In Rust, creating intermediate `Vec` allocations (e.g., using `.collect::<Vec<_>>().into_iter()`) solely for iteration is an anti-pattern that negatively impacts performance.
**Action:** When chaining string splits or processing iterators, consume them directly rather than collecting into intermediate `Vec`s. Use pattern matching on `.next()` calls to avoid allocations when extracting fixed-length items from an iterator.
