use rkyv::{
  with::{ArchiveWith, DeserializeWith, SerializeWith},
  Archive, Deserialize, Serialize,
};

use crate::{CacheableDeserializer, CacheableSerializer, DeserializeError, SerializeError};

pub struct Custom<F> {
  _target: F,
}

pub trait CustomConverter<F> {
  fn from(data: &F) -> Result<Self, SerializeError>
  where
    Self: Sized;
  fn to(self) -> Result<F, DeserializeError>;
}

pub struct CustomResolver<O: Archive> {
  inner: O::Resolver,
  value: O,
}

impl<F, T> ArchiveWith<F> for Custom<T>
where
  T: Archive,
{
  type Archived = T::Archived;
  type Resolver = CustomResolver<T>;
  #[inline]
  unsafe fn resolve_with(
    _field: &F,
    pos: usize,
    resolver: Self::Resolver,
    out: *mut Self::Archived,
  ) {
    let CustomResolver { inner, value } = resolver;
    T::resolve(&value, pos, inner, out)
  }
}

impl<'a, F, T> SerializeWith<F, CacheableSerializer> for Custom<T>
where
  T: Archive + Serialize<CacheableSerializer> + CustomConverter<F>,
{
  #[inline]
  fn serialize_with(
    field: &F,
    s: &mut CacheableSerializer,
  ) -> Result<Self::Resolver, SerializeError> {
    let value = T::from(field)?;
    Ok(CustomResolver {
      inner: T::serialize(&value, s)?,
      value,
    })
  }
}

impl<'a, F, T> DeserializeWith<T::Archived, F, CacheableDeserializer> for Custom<T>
where
  T: Archive + CustomConverter<F>,
  T::Archived: Deserialize<T, CacheableDeserializer>,
{
  #[inline]
  fn deserialize_with(
    field: &T::Archived,
    d: &mut CacheableDeserializer,
  ) -> Result<F, DeserializeError> {
    let data: T = T::Archived::deserialize(field, d)?;
    data.to()
  }
}
