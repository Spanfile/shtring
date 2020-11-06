# shtring

[![](https://img.shields.io/crates/v/shtring)](https://crates.io/crates/shtring) [![](https://docs.rs/shtring/badge.svg)](https://docs.rs/shtring)

Split an input string into arguments by whitespace such that text between matching quotes is combined into a single argument. Additionally, single character escapes are supported and ignored where applicable.

```rust
let input = "Hello world! \"This text will be a single argument.\" 'So \"will\" this.' \\\'Escaped quotes are ignored.\\\'";
let output = shtring::split(input)?;
assert_eq!(output, vec![
    "Hello",
    "world!",
    "This text will be a single argument.",
    "So \"will\" this.",
    "\\\'Escaped",
    "quotes",
    "are",
    "ignored.\\\'",
]);
```
