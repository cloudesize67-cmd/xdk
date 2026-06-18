## 2025-03-02 - Eliminate Vector Allocation in `Casing::Pascal.convert_words`

**Learning:** In `xdk-lib/src/casing.rs`, the `Casing::Pascal` enum variant implemented its text transformation by mapping each word to a new allocated `String` using `pascal_case`, collecting them into an intermediate `Vec<String>`, and finally joining them back into a single `String`. This is inefficient.

**Action:** Replacing this with `.collect::<String>()` instead of `.collect::<Vec<_>>().join("")` completely bypasses the intermediate `Vec` allocation and immediately collapses into a single `String` iteration, saving an allocation step and improving throughput on casing functions.
