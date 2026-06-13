pub mod errors;
pub mod table;
pub use errors::Error;
pub use table::TableV1;
pub mod sys;
pub use sys::{clipboard_lines, stdin_lines};
