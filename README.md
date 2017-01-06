# json-color - A Rust library to colorize JSON strings

## Usage

Add `json-color` to your Cargo.toml as usual.

### Examples

If you don't care about the specific colors used:

```rust
extern crate json_color;

use json_color::Colorizer;

fn main() {
    let colorizer = Colorizer::arbitrary();
    
    if let Some(json_str) = colorizer.colorize_json_str("{ \"foo\": [1, 2.0, false, null] }") {
        println!("{}", json_str);
    }
}
```

If you want to pick specific colors to use:

```rust
extern crate json_color;

use json_color::{Colorizer, Color};

fn main() {
    let colorizer = Colorizer::new()
            .null(Color::Cyan)
            .boolean(Color::Yellow)
            .number(Color::Magenta)
            .string(Color::Green)
            .key(Color::Blue)
            .build();

    if let Some(json_str) = colorizer.colorize_json_str("{ \"foo\": [1, 2.0, false, null] }") {
        println!("{}", json_str);
    }
}
```

## Documentation

Hosted at [docs.rs](https://docs.rs/json-color).
