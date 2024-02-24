//! Basalt library

#![feature(stmt_expr_attributes)]
// Allow adequate module structure
#![allow(clippy::module_inception)]
// Allow disabling clippy for expressions etc.
//#![allow(clippy::missing_errors_doc)]
#![warn(clippy::nursery)]
// Lints
// TODO: warn more allowed-by-default lints
#![warn(clippy::nursery)]
#![warn(missing_docs)]
// TODO: add these
//#![warn(clippy::unwrap_used)]
// Allow annoying stuff
#![allow(clippy::unit_arg)] // Some(func())
#![allow(clippy::nonminimal_bool)] // !result.is_ok()

mod basalt;
mod features;
pub mod ui;

/// Pure reusable library modules, except for ui ones go here. Things like physics calculation or little helpers or traits or macros, etc
mod lib {
  pub mod fdp;
  pub mod graph;
  pub mod path;
  pub mod temp_file;
  #[cfg(test)]
  pub mod test;
}
pub use basalt::BasaltApp;

// ----------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use web::*;
