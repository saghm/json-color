extern crate colored;
extern crate serde_json;

use colored::Colorize;
use serde_json::value::Value;
pub use serde_json::error::Error;

/// Colorize a string of JSON.
/// 
/// # Errors
///
/// An error is returned if the string is invalid JSON or an I/O error occurs.
pub fn colorize_json_str(s: &str) -> Result<String, Error> {
    let value = ::serde_json::from_str(s)?;
    Ok(colorize_json_with_indentation(&value, 0))
}

fn colorize_json_with_indentation(value: &Value, indent_level: u8) -> String {
    match *value {
        Value::Null => "null".cyan().to_string(),
        Value::Bool(true) => "true".yellow().to_string(),
        Value::Bool(false) => "false".yellow().to_string(),
        Value::I64(i) => format!("{}", i).magenta().to_string(),
        Value::U64(u) => format!("{}", u).magenta().to_string(),
        Value::F64(f) => format!("{}", f).magenta().to_string(),
        Value::String(ref s) => format!("\"{}\"", s).green().to_string(),
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
                buf.push_str(&colorize_json_with_indentation(val, indent_level + 1));
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
                buf.push_str(&format!("\"{}\"", key).blue().to_string());
                buf.push_str(": ");
                buf.push_str(&colorize_json_with_indentation(val, indent_level + 1));
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
