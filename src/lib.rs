#![allow(unused_variables)]

mod common;
mod de;
mod error;
mod ser;

pub use de::{from_bytes, Deserializer};
pub use error::{Error, Result};
pub use ser::{to_bytes, Serializer};

#[cfg(test)]
mod tests {
  // foo
}
