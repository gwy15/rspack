use rkyv::{
  with::{ArchiveWith, DeserializeWith, SerializeWith},
  Archive, Deserialize, Serialize,
};

use crate::{CacheableDeserializer, CacheableSerializer, DeserializeError, SerializeError};

pub struct Custom<F> {
  _target: F,
}

pub trait CustomConverter<F, C> {
  fn from(data: &F, ctx: &mut C) -> Result<Self, SerializeError>
  where
    Self: Sized;
  fn to(self, ctx: &mut C) -> Result<F, DeserializeError>;
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

impl<'a, F, T, C> SerializeWith<F, CacheableSerializer<'a, C>> for Custom<T>
where
  T: Archive + Serialize<CacheableSerializer<'a, C>> + CustomConverter<F, C>,
{
  #[inline]
  fn serialize_with(
    field: &F,
    s: &mut CacheableSerializer<'a, C>,
  ) -> Result<Self::Resolver, SerializeError> {
    let value = T::from(field, s.context_mut())?;
    Ok(CustomResolver {
      inner: T::serialize(&value, s)?,
      value,
    })
  }
}

impl<'a, F, T, C> DeserializeWith<T::Archived, F, CacheableDeserializer<'a, C>> for Custom<T>
where
  T: Archive + CustomConverter<F, C>,
  T::Archived: Deserialize<T, CacheableDeserializer<'a, C>>,
{
  #[inline]
  fn deserialize_with(
    field: &T::Archived,
    d: &mut CacheableDeserializer<'a, C>,
  ) -> Result<F, DeserializeError> {
    let data: T = T::Archived::deserialize(field, d)?;
    data.to(d.context_mut())
  }
}
