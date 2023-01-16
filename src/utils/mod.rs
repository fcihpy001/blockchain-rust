use std::fmt::Error;

mod serializer;

pub use serializer::*;

pub type Result<T, E = Error> = core::result::Result<T,E>;
