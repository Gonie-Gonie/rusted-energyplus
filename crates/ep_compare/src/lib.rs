//! Public facade for comparison and tolerance helpers.

#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

mod eio;
mod eso;
mod mtr;
mod series;
mod tolerance;

pub use eio::*;
pub use eso::*;
pub use mtr::*;
pub use series::*;
pub use tolerance::*;

#[cfg(test)]
mod tests;
