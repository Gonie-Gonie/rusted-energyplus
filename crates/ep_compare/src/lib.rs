//! Public facade for comparison and tolerance helpers.

mod eio;
mod eso;
mod series;
mod tolerance;

pub use eio::*;
pub use eso::*;
pub use series::*;
pub use tolerance::*;

#[cfg(test)]
mod tests;
