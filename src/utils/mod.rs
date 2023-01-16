use std::fmt::Error;

pub mod serializer;
pub mod sleddb;

pub use serializer::*;

pub type Result<T, E = Error> = core::result::Result<T,E>;
