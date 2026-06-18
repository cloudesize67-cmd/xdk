## 2024-05-18 - Avoid Vec cloning with slice references
**Learning:** Performance preference: Prioritize using slice references (`&[T]`) over owned vectors (`Vec<T>`) for read-only function arguments to eliminate unnecessary cloning in performance-critical paths. Specifically applied to `clean_operation_id` within the OpenAPI spec parser.
**Action:** Always check function signatures taking `Vec<T>` to see if they only read the data and can take `&[T]` instead to avoid `.clone()` calls.
