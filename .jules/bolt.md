## 2024-05-18 - [Optimization] Removed `.collect::<Vec<_>>().join("")`
**Learning:** Found string concatenation anti-pattern `.collect::<Vec<_>>().join("")` in `Casing::Pascal`. Replaced it with `.collect::<String>()` saving an intermediate vector allocation.
**Action:** When gathering items into a final string, use `.collect::<String>()` instead of `.collect::<Vec<_>>().join("")`.
