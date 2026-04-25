## 2024-04-25 - Prevent intermediate vector allocations during string parsing
**Learning:** Found instances where iterators like `.split()` and `.map()` were needlessly materialized into a `Vec` before being consumed or joined (e.g. `.collect::<Vec<_>>().join("")` and `.collect::<Vec<&str>>().into_iter()`).
**Action:** When parsing strings or joining data in Rust, strive to use intermediate iterators efficiently to avoid allocating temporary heap arrays. Use `.collect::<String>()` instead of `.join("")` and directly chain iterator operations.
