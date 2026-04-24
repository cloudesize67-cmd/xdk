## 2024-04-24 - [Avoid Intermediate Allocations]
**Learning:** In Rust iterator chains like `.split().collect::<Vec<_>>().into_iter()`, an intermediate `Vec` is allocated and populated before being immediately consumed. This wastes memory and time.
**Action:** When performing sequence operations like transformations or string joining, avoid `Vec` or string allocations whenever possible. Use character splits `.split('\n')` directly as iterators, and `collect::<String>()` instead of `join("")` for concatenation.
