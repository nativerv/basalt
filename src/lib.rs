//! Basalt library
//#![allow(clippy::missing_errors_doc)]
#![warn(clippy::nursery)]
// Lints
#![warn(clippy::nursery)]
#![warn(missing_docs)]
// Allow disabling clippy for expressions etc.
#![feature(stmt_expr_attributes)]

mod basalt;
mod fractal_clock;

pub use basalt::BasaltApp;

// ----------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
pub use web::*;
