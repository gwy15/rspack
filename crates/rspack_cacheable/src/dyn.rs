use core::alloc::Layout;
use std::hash::{DefaultHasher, Hash, Hasher};

use ptr_meta::Pointee;
use rkyv::{ser::Serializer, Serialize};
use rkyv_typename::TypeName;

use crate::{CacheableDeserializer, CacheableSerializer, DeserializeError, SerializeError};

/// A trait object forked from rkyv_dyn::SerializeDyn
///
/// This trait will override some internal methods params.
/// 1. serializer from `&mut dyn DynSerializer` to `CacheableSerializer`
/// 2. return Error from `DynError` to `SerializeError`
pub trait SerializeDyn {
  /// Writes the value to the serializer and returns the position it was written to.
  fn serialize_dyn(&self, serializer: &mut CacheableSerializer) -> Result<usize, SerializeError>;

  /// Returns the type ID of the archived version of this type.
  fn archived_type_id(&self) -> u64;
}

impl<T: Serialize<CacheableSerializer>> SerializeDyn for T
where
  T::Archived: TypeName,
{
  fn serialize_dyn(&self, serializer: &mut CacheableSerializer) -> Result<usize, SerializeError> {
    serializer.serialize_value(self)
  }

  fn archived_type_id(&self) -> u64 {
    // TODO rewrite to not depend on TypeName
    let mut hasher = DefaultHasher::new();
    T::Archived::build_type_name(|s| s.hash(&mut hasher));
    hasher.finish()
  }
}

/// A trait object forked from rkyv_dyn::DeserializeDyn
///
/// This trait will override some internal methods params.
/// 1. deserializer from `&mut dyn DynDeserializer` to `CacheableDeserializer`
/// 2. return Error from `DynError` to `DeserializeError`
pub trait DeserializeDyn<T: Pointee + ?Sized> {
  /// Deserializes the given value as a trait object.
  unsafe fn deserialize_dyn(
    &self,
    deserializer: &mut CacheableDeserializer,
    alloc: &mut dyn FnMut(Layout) -> *mut u8,
  ) -> Result<*mut (), DeserializeError>;

  /// Returns the metadata for the deserialized version of this value.
  fn deserialize_dyn_metadata(
    &self,
    deserializer: &mut CacheableDeserializer,
  ) -> Result<T::Metadata, DeserializeError>;
}
