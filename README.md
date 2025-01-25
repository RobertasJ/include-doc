# inline-doc

A working version of

```rust
#[doc = include_str!("path")]
```

Important to note that the path goes from the root of the project.

This project was inspired by [lsp_doc](https://crates.io/crates/lsp_doc)
and made to work on stable.

## Example usage

```rust
#[inline_doc("src/docs/thing/documentation.md")]
struct Thing;

#[inline_doc("src/docs/thing.md")]
fn thing() {}
```

## NOTE

When you change anything you need to restart rust-analyzer
for the documentation to come into effect.
rust-analyzer caches files so it wont update automatically.
