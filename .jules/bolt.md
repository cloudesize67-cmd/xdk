## 2024-05-19 - Iterator cloning optimization

**Learning:** When using split() and mapping, string combinations shouldn't construct intermediate vectors. Avoid intermediate allocations.
**Action:** Replace .collect::<Vec<_>>().join("") with .collect::<String>() for single string concatenation. Replace intermediate collections in iterators if they aren't needed.

## 2024-05-19 - String Split Optimization
**Learning:** Using .split("\n") on a string allocates memory or at least is less efficient than .split('\n').
**Action:** Use character based split() rather than string based splits for single character delimiters.

## 2024-05-19 - Memory and Ownership patterns
**Learning:** In recursive schema mock generation, moving arguments by value prevents extensive .clone() calls.
**Action:** Use value semantics and pattern matching where possible to avoid .clone() in loops.
