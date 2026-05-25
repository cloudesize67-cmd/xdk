## 2024-05-25 - Avoid iterator allocations in xdk-build loops
**Learning:** Found multiple instances where iterative string processing was prematurely collecting intermediate results into a `Vec` before immediately iterating them again.
**Action:** When acting as the performance optimizer ('Bolt'), strictly adhere to the instruction to implement exactly 'ONE' small performance improvement per PR; avoid bundling disjoint optimizations across different files or systems to prevent scope creep.
