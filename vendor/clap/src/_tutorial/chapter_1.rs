//! ## Configuring the Parser
//!
//! You use [`Command`][crate::Command] to start building a parser.
//!
//! ```rust
#![doc = include_str!("../../examples/tutorial_builder/02_apps.rs")]
//! ```
//!
#![doc = include_str!("../../examples/tutorial_builder/02_apps.md")]
//!
//! You can use [`command!()`][crate::command!] to fill these fields in from your `Cargo.toml`
//! file.  **This requires the [`cargo` feature flag][crate::_features].**
//!
//! ```rust
#![doc = include_str!("../../examples/tutorial_builder/02_crate.rs")]
//! ```
#![doc = include_str!("../../examples/tutorial_builder/02_crate.md")]
//!
//! You can use [`Command`][crate::Command] methods to change the application level behavior of
//! clap, like [`Command::next_line_help`].
//!
//! ```rust
#![doc = include_str!("../../examples/tutorial_builder/02_app_settings.rs")]
//! ```
#![doc = include_str!("../../examples/tutorial_builder/02_app_settings.md")]
#![allow(unused_imports)]
use crate::builder::*;

pub use super::chapter_0 as previous;
pub use super::chapter_2 as next;
pub use crate::_tutorial as table_of_contents;
