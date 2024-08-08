use rspack_cacheable::{cacheable, from_bytes, to_bytes};

#[test]
fn test_manual_cacheable_dyn_macro_with_generics() {
  struct Context;

  trait Animal<T = ()>: rspack_cacheable::r#dyn::SerializeDyn {
    fn color(&self) -> &str;
    fn name(&self) -> T;
  }

  const _: () = {
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

    impl<T> ptr_meta::Pointee for dyn Animal<T> {
      type Metadata = ptr_meta::DynMetadata<Self>;
    }

    pub trait DeserializeAnimal<T>: DeserializeDyn<dyn Animal<T>> {}
    impl<T> ptr_meta::Pointee for dyn DeserializeAnimal<T> {
      type Metadata = ptr_meta::DynMetadata<Self>;
    }

    impl<T, O: DeserializeDyn<dyn Animal<T>>> DeserializeAnimal<T> for O {}

    impl<T> TypeName for dyn DeserializeAnimal<T> {
      fn build_type_name<F: FnMut(&str)>(mut f: F) {
        f("dyn DeserializeAnimal<T>");
      }
    }

    impl<T> ArchiveUnsized for dyn Animal<T> {
      type Archived = dyn DeserializeAnimal<T>;
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

    impl<T> ArchivePointee for dyn DeserializeAnimal<T> {
      type ArchivedMetadata = ArchivedDynMetadata<Self>;

      fn pointer_metadata(
        archived: &Self::ArchivedMetadata,
      ) -> <Self as ptr_meta::Pointee>::Metadata {
        archived.pointer_metadata()
      }
    }

    impl<T> SerializeUnsized<CacheableSerializer> for dyn Animal<T> {
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

    impl<T> DeserializeUnsized<dyn Animal<T>, CacheableDeserializer> for dyn DeserializeAnimal<T> {
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
      ) -> Result<<dyn Animal<T> as ptr_meta::Pointee>::Metadata, DeserializeError> {
        self.deserialize_dyn_metadata(&mut deserializer)
      }
    }

    // CheckBytes
    impl<T> LayoutRaw for dyn DeserializeAnimal<T> {
      fn layout_raw(
        metadata: <Self as ptr_meta::Pointee>::Metadata,
      ) -> Result<Layout, LayoutError> {
        Ok(metadata.layout())
      }
    }
    impl<T, C: DynContext> CheckBytes<C> for dyn DeserializeAnimal<T> {
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

  impl Animal<&'static str> for Dog {
    fn color(&self) -> &str {
      &self.color
    }
    fn name(&self) -> &'static str {
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

    register_impl!(ArchivedDog as <dyn Animal<&'static str> as ArchiveUnsized>::Archived);

    impl DeserializeDyn<dyn Animal<&'static str>> for Archived<Dog>
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
      ) -> Result<<dyn Animal<&'static str> as ptr_meta::Pointee>::Metadata, DeserializeError>
      {
        unsafe {
          Ok(core::mem::transmute(ptr_meta::metadata(
            core::ptr::null::<Dog>() as *const dyn Animal<&'static str>,
          )))
        }
      }
    }
  };

  #[cacheable]
  struct Cat {
    color: String,
  }

  impl rspack_cacheable::__private::rkyv_typename::TypeName for ArchivedCat {
    fn build_type_name<F: FnMut(&str)>(mut f: F) {
      f("Animal Cat");
    }
  }

  impl Animal<String> for Cat {
    fn color(&self) -> &str {
      &self.color
    }
    fn name(&self) -> String {
      String::from("cat")
    }
  }

  const _: () = {
    use core::alloc::Layout;

    use rspack_cacheable::__private::{
      ptr_meta,
      rkyv::{ArchiveUnsized, Archived, Deserialize},
      rkyv_dyn::{self, register_impl},
    };
    use rspack_cacheable::{r#dyn::DeserializeDyn, CacheableDeserializer, DeserializeError};

    register_impl!(ArchivedCat as <dyn Animal<String> as ArchiveUnsized>::Archived);

    impl DeserializeDyn<dyn Animal<String>> for Archived<Cat>
    where
      Archived<Cat>: Deserialize<Cat, CacheableDeserializer>,
    {
      unsafe fn deserialize_dyn(
        &self,
        deserializer: &mut CacheableDeserializer,
        alloc: &mut dyn FnMut(Layout) -> *mut u8,
      ) -> Result<*mut (), DeserializeError> {
        let result = alloc(Layout::new::<Cat>()).cast::<Cat>();
        assert!(!result.is_null());
        result.write(self.deserialize(deserializer)?);
        Ok(result as *mut ())
      }

      fn deserialize_dyn_metadata(
        &self,
        _: &mut CacheableDeserializer,
      ) -> Result<<dyn Animal<String> as ptr_meta::Pointee>::Metadata, DeserializeError> {
        unsafe {
          Ok(core::mem::transmute(ptr_meta::metadata(
            core::ptr::null::<Cat>() as *const dyn Animal<String>,
          )))
        }
      }
    }
  };

  #[cacheable]
  struct Data {
    animal_1: Box<dyn Animal<&'static str>>,
    animal_2: Box<dyn Animal<String>>,
  }

  let data = Data {
    animal_1: Box::new(Dog {
      color: String::from("black"),
    }),
    animal_2: Box::new(Cat {
      color: String::from("white"),
    }),
  };
  assert_eq!(data.animal_1.name(), "dog");
  assert_eq!(data.animal_1.color(), "black");
  assert_eq!(data.animal_2.name(), "cat");
  assert_eq!(data.animal_2.color(), "white");
  let mut ctx = Context {};
  let bytes = to_bytes(&data, &mut ctx).unwrap();
  let deserialize_data = from_bytes::<Data, Context>(&bytes, &mut ctx).unwrap();
  assert_eq!(deserialize_data.animal_1.name(), "dog");
  assert_eq!(deserialize_data.animal_1.color(), "black");
  assert_eq!(deserialize_data.animal_2.name(), "cat");
  assert_eq!(deserialize_data.animal_2.color(), "white");
}
