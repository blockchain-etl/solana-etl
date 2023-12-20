# Contribution & Style Guide:

1. Pull requests should result in code that does not produce any errors when compiled.
2. Pull requests should result in code that does not produce any warnings when compiled.
3. Run `cargo fmt` before opening a pull request.
4. Run `cargo clippy` and resolve any warnings.
5. All functions should have a comment directly above that explains what it does at a high level. This comment should use the triple forward-slash `///` for compatibility with rustdoc.
6. Each module should contain a high-level description of what the module is intended for. Make sure to use `//!` in the mod.rs file, and each respective rust source file.
7. No more than 1 line of whitespace at a time. 
8. Side-effects, assumptions, and “gotchas” should be accompanied by a `NOTE:` comment. 
9. Try to follow a functional style, except for when calling async functions:
- Use constants and immutable variables.
- Use `Iterator` methods like `filter()`, `map()`, and `fold()`.
