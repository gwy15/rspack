use std::marker::PhantomData;

use rkyv::{
  collections::util::validation::ArchivedEntryError,
  out_field,
  ser::{ScratchSpace, Serializer},
  validation::ArchiveContext,
  vec::{ArchivedVec, VecResolver},
  with::{ArchiveWith, DeserializeWith, SerializeWith},
  Archive, CheckBytes, Fallible, Serialize,
};

use crate::{CacheableDeserializer, DeserializeError};

pub struct AsMap<WK, WV> {
  _key: WK,
  _value: WV,
}

pub trait AsMapConverter {
  type Key;
  type Value;
  fn len(&self) -> usize;
  fn iter(&self) -> impl ExactSizeIterator<Item = (&Self::Key, &Self::Value)>;
  fn from(
    data: impl ExactSizeIterator<Item = Result<(Self::Key, Self::Value), DeserializeError>>,
  ) -> Result<Self, DeserializeError>
  where
    Self: Sized;
}

pub struct Entry<K, V, WK, WV> {
  pub key: K,
  pub value: V,
  // with for key
  _key: PhantomData<WK>,
  // with for value
  _value: PhantomData<WV>,
}

impl<WK, WV, K, V> Archive for Entry<&'_ K, &'_ V, WK, WV>
where
  WK: ArchiveWith<K>,
  WV: ArchiveWith<V>,
{
  type Archived = Entry<WK::Archived, WV::Archived, WK, WV>;
  type Resolver = (WK::Resolver, WV::Resolver);

  #[inline]
  unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
    let (fp, fo) = out_field!(out.key);
    WK::resolve_with(self.key, pos + fp, resolver.0, fo);

    let (fp, fo) = out_field!(out.value);
    WV::resolve_with(self.value, pos + fp, resolver.1, fo);
  }
}

impl<WK, WV, K, V, S: Fallible + ?Sized> Serialize<S> for Entry<&'_ K, &'_ V, WK, WV>
where
  WK: SerializeWith<K, S>,
  WV: SerializeWith<V, S>,
{
  #[inline]
  fn serialize(&self, serializer: &mut S) -> Result<Self::Resolver, S::Error> {
    Ok((
      WK::serialize_with(self.key, serializer)?,
      WV::serialize_with(self.value, serializer)?,
    ))
  }
}

impl<K, V, WK, WV, C> CheckBytes<C> for Entry<K, V, WK, WV>
where
  K: CheckBytes<C>,
  V: CheckBytes<C>,
  C: ArchiveContext + ?Sized,
{
  type Error = ArchivedEntryError<K::Error, V::Error>;

  #[inline]
  unsafe fn check_bytes<'a>(value: *const Self, context: &mut C) -> Result<&'a Self, Self::Error> {
    K::check_bytes(core::ptr::addr_of!((*value).key), context)
      .map_err(ArchivedEntryError::KeyCheckError)?;
    V::check_bytes(core::ptr::addr_of!((*value).value), context)
      .map_err(ArchivedEntryError::ValueCheckError)?;
    Ok(&*value)
  }
}

impl<T, K, V, WK, WV> ArchiveWith<T> for AsMap<WK, WV>
where
  T: AsMapConverter<Key = K, Value = V>,
  WK: ArchiveWith<K>,
  WV: ArchiveWith<V>,
{
  type Archived = ArchivedVec<Entry<WK::Archived, WV::Archived, WK, WV>>;
  type Resolver = VecResolver;

  unsafe fn resolve_with(
    field: &T,
    pos: usize,
    resolver: Self::Resolver,
    out: *mut Self::Archived,
  ) {
    ArchivedVec::resolve_from_len(field.len(), pos, resolver, out)
  }
}

impl<T, K, V, WK, WV, S> SerializeWith<T, S> for AsMap<WK, WV>
where
  T: AsMapConverter<Key = K, Value = V>,
  WK: ArchiveWith<K>,
  WV: ArchiveWith<V>,
  S: Fallible + ScratchSpace + Serializer + ?Sized,
  for<'a> Entry<&'a K, &'a V, WK, WV>: Serialize<S>,
{
  fn serialize_with(field: &T, s: &mut S) -> Result<Self::Resolver, S::Error> {
    ArchivedVec::serialize_from_iter(
      field.iter().map(|(key, value)| Entry {
        key,
        value,
        _key: PhantomData::<WK>::default(),
        _value: PhantomData::<WV>::default(),
      }),
      s,
    )
  }
}

impl<'a, K, V, WK, WV, T, C>
  DeserializeWith<
    ArchivedVec<Entry<WK::Archived, WV::Archived, WK, WV>>,
    T,
    CacheableDeserializer<'a, C>,
  > for AsMap<WK, WV>
where
  T: AsMapConverter<Key = K, Value = V>,
  K: std::hash::Hash + Eq,
  WK: ArchiveWith<K> + DeserializeWith<WK::Archived, K, CacheableDeserializer<'a, C>>,
  WV: ArchiveWith<V> + DeserializeWith<WV::Archived, V, CacheableDeserializer<'a, C>>,
{
  fn deserialize_with(
    field: &ArchivedVec<Entry<WK::Archived, WV::Archived, WK, WV>>,
    deserializer: &mut CacheableDeserializer<'a, C>,
  ) -> Result<T, DeserializeError> {
    T::from(field.iter().map(|Entry { key, value, .. }| {
      Ok((
        WK::deserialize_with(key, deserializer)?,
        WV::deserialize_with(value, deserializer)?,
      ))
    }))
  }
}

// for rustc_hash::FxHashMap
impl<K, V> AsMapConverter for rustc_hash::FxHashMap<K, V>
where
  K: std::cmp::Eq + std::hash::Hash,
{
  type Key = K;
  type Value = V;
  fn len(&self) -> usize {
    self.len()
  }
  fn iter(&self) -> impl ExactSizeIterator<Item = (&Self::Key, &Self::Value)> {
    self.iter()
  }
  fn from(
    data: impl ExactSizeIterator<Item = Result<(Self::Key, Self::Value), DeserializeError>>,
  ) -> Result<Self, DeserializeError> {
    data.collect::<Result<rustc_hash::FxHashMap<_, _>, DeserializeError>>()
  }
}
