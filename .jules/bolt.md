## 2026-05-14 - [Intermediate Allocations]
**Learning:** String concatenation using `.collect::<Vec<_>>().join("")` creates unnecessary intermediate allocations compared to `.collect::<String>()`.
**Action:** Always prefer `.collect::<String>()` for iterator-based string building.
