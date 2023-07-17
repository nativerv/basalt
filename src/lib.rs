//! Basalt library

//#![allow(clippy::missing_errors_doc)]
#![warn(clippy::nursery)]
// Lints
#![warn(clippy::nursery)]
#![warn(missing_docs)]
// Allow disabling clippy for expressions etc.
#![feature(stmt_expr_attributes)]
#[macro_use] extern crate lazy_static;

mod basalt;
mod features;
mod fractal_clock;


mod lib {
  pub mod graph;
  #[macro_use]
  pub mod serial_test;
}
pub use basalt::BasaltApp;

// ----------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(target_arch = "wasm32")]
pub use web::*;
