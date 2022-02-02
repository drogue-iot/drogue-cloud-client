//! A client for the Drogue IoT Cloud APIs.

pub mod core;
pub mod error;
pub mod meta;
#[cfg(feature = "openid")]
pub mod openid;
pub mod registry;

mod serde;
mod translator;

pub use translator::*;
