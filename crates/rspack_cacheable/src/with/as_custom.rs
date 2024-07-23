use rkyv::{
  with::{ArchiveWith, DeserializeWith, SerializeWith},
  Archive, Deserialize, Serialize,
};

use crate::{CacheableDeserializer, CacheableSerializer, DeserializeError, SerializeError};

pub struct AsCustom;

pub trait AsCustomConverter {
  type S;
  type Context;
  fn to(&self, ctx: &mut Self::Context) -> Result<Self::S, SerializeError>;
  fn from(data: Self::S, ctx: &mut Self::Context) -> Result<Self, DeserializeError>
  where
    Self: Sized;
}

pub struct AsCustomResolver<O: Archive> {
  inner: O::Resolver,
  value: O,
}

impl<T, O> ArchiveWith<T> for AsCustom
where
  T: AsCustomConverter<S = O>,
  O: Archive,
{
  type Archived = O::Archived;
  type Resolver = AsCustomResolver<O>;
  #[inline]
  unsafe fn resolve_with(
    _field: &T,
    pos: usize,
    resolver: Self::Resolver,
    out: *mut Self::Archived,
  ) {
    let AsCustomResolver { inner, value } = resolver;
    O::resolve(&value, pos, inner, out)
  }
}

impl<'a, T, O, C> SerializeWith<T, CacheableSerializer<'a, C>> for AsCustom
where
  T: AsCustomConverter<S = O, Context = C>,
  O: Archive + Serialize<CacheableSerializer<'a, C>>,
{
  #[inline]
  fn serialize_with(
    field: &T,
    s: &mut CacheableSerializer<'a, C>,
  ) -> Result<Self::Resolver, SerializeError> {
    let value = field.to(s.get_context())?;
    Ok(AsCustomResolver {
      inner: O::serialize(&value, s)?,
      value,
    })
  }
}

impl<'a, T, O, C> DeserializeWith<O::Archived, T, CacheableDeserializer<'a, C>> for AsCustom
where
  T: AsCustomConverter<S = O, Context = C>,
  O: Archive,
  O::Archived: Deserialize<O, CacheableDeserializer<'a, C>>,
{
  #[inline]
  fn deserialize_with(
    field: &O::Archived,
    d: &mut CacheableDeserializer<'a, C>,
  ) -> Result<T, DeserializeError> {
    let data = O::Archived::deserialize(field, d)?;
    AsCustomConverter::from(data, d.get_context())
  }
}
