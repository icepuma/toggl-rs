#[cfg(feature = "client")]
pub mod client;

pub mod error;
pub mod model;

#[cfg(all(test, feature = "client"))]
mod tests;
