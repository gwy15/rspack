use std::sync::Arc;

use rkyv::{
  ser::{ScratchSpace, Serializer},
  with::{ArchiveWith, DeserializeWith, SerializeWith},
  Fallible,
};
use rspack_sources::Source;

use super::AsPreset;

// trait Animal: rspack_cacheable::r#dyn::SerializeDyn {
//   fn color(&self) -> &str;
//   fn name(&self) -> &str;
//}

impl ArchiveWith<Arc<dyn Source>> for AsPreset {
  type Archived = ();
  type Resolver = ();

  #[inline]
  unsafe fn resolve_with(
    _field: &Arc<dyn Source>,
    _pos: usize,
    _resolver: Self::Resolver,
    _out: *mut Self::Archived,
  ) {
    ()
  }
}

impl<S> SerializeWith<Arc<dyn Source>, S> for AsPreset
where
  S: ?Sized + Serializer + ScratchSpace,
{
  #[inline]
  fn serialize_with(
    _field: &Arc<dyn Source>,
    _serializer: &mut S,
  ) -> Result<Self::Resolver, S::Error> {
    todo!()
  }
}

impl<D> DeserializeWith<(), Arc<dyn Source>, D> for AsPreset
where
  D: ?Sized + Fallible,
{
  #[inline]
  fn deserialize_with(_field: &(), _: &mut D) -> Result<Arc<dyn Source>, D::Error> {
    todo!()
  }
}

/*const _: () = {
  use core::alloc::Layout;
  use std::alloc::LayoutError;

  use rspack_cacheable::__private::{
    ptr_meta,
    rkyv::{
      validation::LayoutRaw, ArchivePointee, ArchiveUnsized, ArchivedMetadata, CheckBytes,
      DeserializeUnsized, SerializeUnsized,
    },
    rkyv_dyn::{
      validation::{CheckDynError, DynContext, CHECK_BYTES_REGISTRY},
      ArchivedDynMetadata,
    },
    rkyv_typename::TypeName,
  };
  use rspack_cacheable::{
    r#dyn::DeserializeDyn, CacheableDeserializer, CacheableSerializer, DeserializeError,
    SerializeError,
  };

  impl ptr_meta::Pointee for dyn Animal {
    type Metadata = ptr_meta::DynMetadata<Self>;
  }

  pub trait DeserializeAnimal: DeserializeDyn<dyn Animal> {}
  impl ptr_meta::Pointee for dyn DeserializeAnimal {
    type Metadata = ptr_meta::DynMetadata<Self>;
  }

  impl<T: DeserializeDyn<dyn Animal>> DeserializeAnimal for T {}

  impl TypeName for dyn DeserializeAnimal {
    fn build_type_name<F: FnMut(&str)>(mut f: F) {
      f("dyn DeserializeAnimal");
    }
  }

  impl ArchiveUnsized for dyn Animal {
    type Archived = dyn DeserializeAnimal;
    type MetadataResolver = ();

    unsafe fn resolve_metadata(
      &self,
      _: usize,
      _: Self::MetadataResolver,
      out: *mut ArchivedMetadata<Self>,
    ) {
      ArchivedDynMetadata::emplace(self.archived_type_id(), out);
    }
  }

  impl ArchivePointee for dyn DeserializeAnimal {
    type ArchivedMetadata = ArchivedDynMetadata<Self>;

    fn pointer_metadata(
      archived: &Self::ArchivedMetadata,
    ) -> <Self as ptr_meta::Pointee>::Metadata {
      archived.pointer_metadata()
    }
  }

  impl SerializeUnsized<CacheableSerializer> for dyn Animal {
    fn serialize_unsized(
      &self,
      mut serializer: &mut CacheableSerializer,
    ) -> Result<usize, SerializeError> {
      self.serialize_dyn(&mut serializer)
    }

    fn serialize_metadata(
      &self,
      _: &mut CacheableSerializer,
    ) -> Result<Self::MetadataResolver, SerializeError> {
      Ok(())
    }
  }

  impl DeserializeUnsized<dyn Animal, CacheableDeserializer> for dyn DeserializeAnimal {
    unsafe fn deserialize_unsized(
      &self,
      mut deserializer: &mut CacheableDeserializer,
      mut alloc: impl FnMut(Layout) -> *mut u8,
    ) -> Result<*mut (), DeserializeError> {
      self.deserialize_dyn(&mut deserializer, &mut alloc)
    }

    fn deserialize_metadata(
      &self,
      mut deserializer: &mut CacheableDeserializer,
    ) -> Result<<dyn Animal as ptr_meta::Pointee>::Metadata, DeserializeError> {
      self.deserialize_dyn_metadata(&mut deserializer)
    }
  }

  // CheckBytes
  impl LayoutRaw for dyn DeserializeAnimal {
    fn layout_raw(metadata: <Self as ptr_meta::Pointee>::Metadata) -> Result<Layout, LayoutError> {
      Ok(metadata.layout())
    }
  }
  impl<C: DynContext> CheckBytes<C> for dyn DeserializeAnimal {
    type Error = CheckDynError;
    #[inline]
    unsafe fn check_bytes<'a>(
      value: *const Self,
      context: &mut C,
    ) -> Result<&'a Self, Self::Error> {
      let vtable = core::mem::transmute(ptr_meta::metadata(value));
      if let Some(validation) = CHECK_BYTES_REGISTRY.get(vtable) {
        (validation.check_bytes_dyn)(value.cast(), context as &mut dyn DynContext)?;
        Ok(&*value)
      } else {
        Err(CheckDynError::InvalidMetadata(vtable as usize as u64))
      }
    }
  }
};

#[cacheable]
struct Dog {
  color: String,
}

impl rspack_cacheable::__private::rkyv_typename::TypeName for ArchivedDog {
  fn build_type_name<F: FnMut(&str)>(mut f: F) {
    f("Animal Dog");
  }
}

impl Animal for Dog {
  fn color(&self) -> &str {
    &self.color
  }
  fn name(&self) -> &str {
    "dog"
  }
}

const _: () = {
  use core::alloc::Layout;

  use rspack_cacheable::__private::{
    inventory, ptr_meta,
    rkyv::{ArchiveUnsized, Archived, Deserialize},
    rkyv_dyn::{self, register_impl},
  };
  use rspack_cacheable::{r#dyn::DeserializeDyn, CacheableDeserializer, DeserializeError};

  register_impl!(ArchivedDog as <dyn Animal as ArchiveUnsized>::Archived);

  impl DeserializeDyn<dyn Animal> for Archived<Dog>
  where
    Archived<Dog>: Deserialize<Dog, CacheableDeserializer>,
  {
    unsafe fn deserialize_dyn(
      &self,
      deserializer: &mut CacheableDeserializer,
      alloc: &mut dyn FnMut(Layout) -> *mut u8,
    ) -> Result<*mut (), DeserializeError> {
      let result = alloc(Layout::new::<Dog>()).cast::<Dog>();
      assert!(!result.is_null());
      result.write(self.deserialize(deserializer)?);
      Ok(result as *mut ())
    }

    fn deserialize_dyn_metadata(
      &self,
      _: &mut CacheableDeserializer,
    ) -> Result<<dyn Animal as ptr_meta::Pointee>::Metadata, DeserializeError> {
      unsafe {
        Ok(core::mem::transmute(ptr_meta::metadata(
          core::ptr::null::<Dog>() as *const dyn Animal,
        )))
      }
    }
  }
};*/
