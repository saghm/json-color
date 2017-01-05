# json-color - A Rust library to colorize JSON strings

## Usage

Add `json-color` to your Cargo.toml as usual, and then import and call the `colorize_json_string` function:

```rust
extern crate json_color;

use json_color::colorize_json_str

...

if let Ok(colored_json) = colorize_json_str("{ \"foo\": [1, 2.0, false, null] }") {
    println!("{}", colored_json);
}
```

That's it!

## Documentation

Hosted at [docs.rs](https://docs.rs/json-color).
