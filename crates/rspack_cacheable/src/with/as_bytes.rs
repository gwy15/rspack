use rkyv::{
  vec::{ArchivedVec, VecResolver},
  with::{ArchiveWith, DeserializeWith, SerializeWith},
};

use crate::{CacheableDeserializer, CacheableSerializer, DeserializeError, SerializeError};

pub struct AsBytes;

pub trait AsBytesConverter<C> {
  // todo change return to Result<Cow<Vec<u8>>, SerializeError>
  fn to_bytes(&self, context: &mut C) -> Result<Vec<u8>, SerializeError>;
  fn from_bytes(s: &[u8], context: &mut C) -> Result<Self, DeserializeError>
  where
    Self: Sized;
}

pub struct AsBytesResolver {
  inner: VecResolver,
  len: usize,
}

impl<T> ArchiveWith<T> for AsBytes {
  type Archived = ArchivedVec<u8>;
  type Resolver = AsBytesResolver;

  #[inline]
  unsafe fn resolve_with(
    _field: &T,
    pos: usize,
    resolver: Self::Resolver,
    out: *mut Self::Archived,
  ) {
    ArchivedVec::resolve_from_len(resolver.len, pos, resolver.inner, out)
  }
}

impl<'a, T, C> SerializeWith<T, CacheableSerializer<'a, C>> for AsBytes
where
  T: AsBytesConverter<C>,
{
  #[inline]
  fn serialize_with(
    field: &T,
    serializer: &mut CacheableSerializer<'a, C>,
  ) -> Result<Self::Resolver, SerializeError> {
    let bytes = &field.to_bytes(serializer.context_mut())?;
    Ok(AsBytesResolver {
      inner: ArchivedVec::serialize_from_slice(bytes, serializer)?,
      len: bytes.len(),
    })
  }
}

impl<'a, T, C> DeserializeWith<ArchivedVec<u8>, T, CacheableDeserializer<'a, C>> for AsBytes
where
  T: AsBytesConverter<C>,
{
  #[inline]
  fn deserialize_with(
    field: &ArchivedVec<u8>,
    de: &mut CacheableDeserializer<'a, C>,
  ) -> Result<T, DeserializeError> {
    AsBytesConverter::from_bytes(field, de.context_mut())
  }
}

// for rspack_source
use std::sync::Arc;

use rspack_sources::RawSource;

use crate::utils::{TypeWrapper, TypeWrapperRef};

impl<C> AsBytesConverter<C> for rspack_sources::BoxSource {
  fn to_bytes(&self, context: &mut C) -> Result<Vec<u8>, SerializeError> {
    let inner = self.as_ref().as_any();
    let mut data: Option<TypeWrapperRef> = None;
    if let Some(raw_source) = inner.downcast_ref::<rspack_sources::RawSource>() {
      match raw_source {
        RawSource::Buffer(buf) => {
          // TODO try avoid clone
          data = Some(TypeWrapperRef {
            type_name: "RawSource::Buffer",
            bytes: buf,
          });
        }
        RawSource::Source(source) => {
          data = Some(TypeWrapperRef {
            type_name: "RawSource::Source",
            bytes: source.as_bytes(),
          });
        }
      }
      //    } else if let Some() = inner.downcast_ref::<rspack_sources::RawSource>() {
    }

    if let Some(data) = data {
      crate::to_bytes(&data, context)
    } else {
      panic!("unsupport box source")
    }
  }
  fn from_bytes(s: &[u8], context: &mut C) -> Result<Self, DeserializeError>
  where
    Self: Sized,
  {
    let TypeWrapper { type_name, bytes } = crate::from_bytes(s, context)?;
    Ok(match type_name.as_str() {
      "RawSource::Buffer" => Arc::new(RawSource::Buffer(bytes)),
      "RawSource::Source" => Arc::new(RawSource::Source(
        String::from_utf8(bytes).expect("unexpect bytes"),
      )),
      _ => {
        panic!("unsupport box source")
      }
    })
  }
}
