## 2024-05-24 - Avoid Vec clone and reallocation using retain and slices
**Learning:** In Rust string/vec processing loops, using `.retain()` is significantly more efficient than allocating a new vector and pushing to it. Additionally, passing `&[String]` instead of `Vec<String>` eliminates the need to `.clone()` the vector on the caller side.
**Action:** Always prefer slice references (`&[T]`) for read-only function arguments, and use `retain` or other in-place iterators over creating new vectors where possible.
