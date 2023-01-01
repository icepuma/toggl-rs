#[cfg(feature = "client")]
pub mod client;

pub mod error;
pub mod model;

#[cfg(test)]
mod client_tests;
