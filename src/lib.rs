extern crate colored;
extern crate serde_json;

use colored::Colorize;
use serde_json::value::Value;
pub use serde_json::error::Error;

macro_rules! colorize {
    ($val:expr, $color:expr) => {
        colorize_str(&format!("{}", $val), $color)
    }
}

macro_rules! colorize_with_quotes {
    ($val:expr, $color:expr) => {
        colorize_str(&format!("\"{}\"", $val), $color)
    }
}

fn colorize_str(s: &str, color: &Color) -> String {
    match *color {
        Color::Black => s.black().to_string(),
        Color::Blue => s.blue().to_string(),
        Color::Cyan => s.cyan().to_string(),
        Color::Green => s.green().to_string(),
        Color::Magenta => s.magenta().to_string(),
        Color::Purple => s.purple().to_string(),
        Color::Red => s.red().to_string(),
        Color::White => s.white().to_string(),
        Color::Yellow => s.yellow().to_string(),

        // Default color
        Color::Plain => s.to_string(),
    }
}

#[derive(Clone)]
pub enum Color {
    Black,
    Blue,
    Cyan,
    Green,
    Magenta,
    Purple,
    Red,
    White,
    Yellow,

    // Default color
    Plain,
}

impl Default for Color {
    fn default() -> Self {
        Color::Plain
    }
}

#[derive(Default)]
pub struct Colorizer {
    null: Color,
    boolean: Color,
    number: Color,
    string: Color,
    key: Color,
}

impl Colorizer {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn null(&mut self, color: Color) -> &mut Self {
        self.null = color;
        self
    }

    pub fn boolean(&mut self, color: Color) -> &mut Self {
        self.boolean = color;
        self
    }

    pub fn number(&mut self, color: Color) -> &mut Self {
        self.number = color;
        self
    }

    pub fn string(&mut self, color: Color) -> &mut Self {
        self.string = color;
        self
    }

    pub fn key(&mut self, color: Color) -> &mut Self {
        self.key = color;
        self
    }

    pub fn arbitrary() -> Self {
        let mut colorizer = Colorizer::new();
        colorizer.null(Color::Cyan).boolean(Color::Yellow).number(Color::Magenta).string(Color::Green).key(Color::Blue);

        colorizer
    } 

    /// Colorize a string of JSON.
    ///
    /// # Errors
    ///
    /// An error is returned if the string is invalid JSON or an I/O error occurs.
    pub fn colorize_json_str(&self, s: &str) -> Result<String, Error> {
        let value = ::serde_json::from_str(s)?;
        Ok(self.colorize_json_with_indentation(&value, 0))
    }

    fn colorize_json_with_indentation(&self, value: &Value, indent_level: u8) -> String {
        match *value {
            Value::Null => colorize_str("null", &self.null),
            Value::Bool(true) => colorize_str("true", &self.boolean),
            Value::Bool(false) => colorize_str("false", &self.boolean),
            Value::I64(i) => colorize!(i, &self.number),
            Value::U64(u) => colorize!(u, &self.number),
            Value::F64(f) => colorize!(f, &self.number), 
            Value::String(ref s) => colorize_with_quotes!(s, &self.string),
            Value::Array(ref values) => {
                let indentation: String = (0..indent_level * 2).map(|_| ' ').collect();
                let mut buf = String::new();

                buf.push('[');

                for (i, val) in values.iter().enumerate() {
                    if i != 0 {
                        buf.push(',');
                    }

                    buf.push('\n');
                    buf.push_str(&indentation);
                    buf.push_str("  ");
                    buf.push_str(&self.colorize_json_with_indentation(val, indent_level + 1));
                }

                if !values.is_empty() {
                    buf.push('\n');
                    buf.push_str(&indentation);
                }

                buf.push(']');
                buf
            }
            Value::Object(ref obj) => {
                let indentation: String = (0..indent_level * 2).map(|_| ' ').collect();
                let mut buf = String::new();

                buf.push('{');

                for (i, (key, val)) in obj.iter().enumerate() {
                    if i != 0 {
                        buf.push(',');
                    }

                    buf.push('\n');
                    buf.push_str(&indentation);
                    buf.push_str("  ");
                    buf.push_str(&colorize_with_quotes!(key, &self.key));
                    buf.push_str(": ");
                    buf.push_str(&self.colorize_json_with_indentation(val, indent_level + 1));
                }

                if !obj.is_empty() {
                    buf.push('\n');
                    buf.push_str(&indentation);
                }

                buf.push('}');
                buf
            }
        }
    }
}
