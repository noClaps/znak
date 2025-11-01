//! An HTML parsing and manipulation library.
//!
//! You can add this library to your project with:
//!
//! ```bash
//! cargo add --git https://github.com/noClaps/znak html
//! ```

mod macros;
mod parser;

pub use parser::{Node, parse};
