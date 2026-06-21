## 2024-05-23 - Verifying multiline code segments for optimizations
**Learning:** Standard grep queries for multiline code snippets (like `collect::<Vec<_>>().join("")`) often fail because they search line-by-line, causing missed opportunities or unverified code edits. Rust code formatting frequently breaks method chains across multiple lines.
**Action:** Always use tools like `sed -n 'start,endp' file`, `cat` with manual inspection, or `grep -C` to verify the exact structure of multiline code blocks *before* creating a plan to modify them with `replace_with_git_merge_diff`.
