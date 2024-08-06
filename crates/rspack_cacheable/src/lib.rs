pub use rspack_macros::{cacheable, cacheable_dyn};
pub mod utils;
pub mod with;

mod deserialize;
mod serialize;

#[doc(hidden)]
pub mod __private {
  #[doc(hidden)]
  pub extern crate inventory;
  #[doc(hidden)]
  pub extern crate once_cell;
  #[doc(hidden)]
  pub extern crate ptr_meta;
  #[doc(hidden)]
  pub extern crate rkyv;
  #[doc(hidden)]
  pub extern crate rkyv_dyn;
  #[doc(hidden)]
  pub extern crate rkyv_typename;
}

pub use deserialize::{from_bytes, CacheableDeserializer, DeserializeError};
pub use serialize::{to_bytes, CacheableSerializer, SerializeError};
