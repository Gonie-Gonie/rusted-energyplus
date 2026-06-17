//! Zone equipment demand and dispatch state used by compatibility-mode HVAC components.

mod demand;
mod dispatch;

pub use demand::*;
pub use dispatch::*;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
