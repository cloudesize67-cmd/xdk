## 2026-05-03 - [Optimize Output Parsing]
**Learning:** Found string-based splits being collected to intermediate `Vec` allocations inside tight loops for terminal logging. This caused unneeded O(N) memory allocations per log output.
**Action:** Always prefer using `split(char)` directly into `.for_each()` or a loop over iterator, avoiding `.collect::<Vec<_>>()` whenever all we need is sequential iteration or pattern matching (e.g. `let (Some(a), Some(b), None) = (iter.next(), iter.next(), iter.next())`).
