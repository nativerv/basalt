//! Demo app for egui
//#![allow(clippy::missing_errors_doc)]
#![warn(clippy::nursery)]
// Lints
#![warn(clippy::nursery)]
#![warn(missing_docs)]

mod basalt;

pub use basalt::BasaltApp;

// ----------------------------------------------------------------------------

#[cfg(target_arch = "wasm32")]
mod web;

#[cfg(target_arch = "wasm32")]
pub use web::*;
