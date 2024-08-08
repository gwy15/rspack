pub use rkyv::with::Skip;
use rkyv::{
  with::{ArchiveWith, DeserializeWith, SerializeWith},
  Fallible,
};

use crate::{CacheableDeserializer, DeserializeError};

pub struct SkipSerialize;

pub trait SkipSerializeConverter {
  fn deserialize(d: &mut CacheableDeserializer) -> Result<Self, DeserializeError>
  where
    Self: Sized;
}

impl<F> ArchiveWith<F> for SkipSerialize {
  type Archived = ();
  type Resolver = ();

  unsafe fn resolve_with(_: &F, _: usize, _: Self::Resolver, _: *mut Self::Archived) {}
}

impl<F, S: Fallible + ?Sized> SerializeWith<F, S> for SkipSerialize {
  fn serialize_with(_: &F, _: &mut S) -> Result<(), S::Error> {
    Ok(())
  }
}

impl<F> DeserializeWith<(), F, CacheableDeserializer> for SkipSerialize
where
  F: SkipSerializeConverter,
{
  fn deserialize_with(_: &(), d: &mut CacheableDeserializer) -> Result<F, DeserializeError> {
    F::deserialize(d)
  }
}
