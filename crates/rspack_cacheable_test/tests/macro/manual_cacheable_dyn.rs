use rspack_cacheable::{cacheable, from_bytes, to_bytes};

#[test]
fn test_manual_cacheable_dyn_macro() {
  struct Context;

  trait Animal: rspack_cacheable::__private::rkyv_dyn::SerializeDyn {
    fn color(&self) -> &str;
    fn name(&self) -> &str;
  }

  const _: () = {
    use core::alloc::Layout;
    use std::alloc::LayoutError;

    use rspack_cacheable::__private::{
      ptr_meta,
      rkyv::{
        ser::{ScratchSpace, Serializer},
        validation::LayoutRaw,
        ArchivePointee, ArchiveUnsized, ArchivedMetadata, CheckBytes, DeserializeUnsized, Fallible,
        SerializeUnsized,
      },
      rkyv_dyn::{
        validation::{CheckDynError, DynContext, CHECK_BYTES_REGISTRY},
        ArchivedDynMetadata, DeserializeDyn,
      },
      rkyv_typename::TypeName,
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

    impl<S: ScratchSpace + Serializer + ?Sized> SerializeUnsized<S> for dyn Animal {
      fn serialize_unsized(&self, mut serializer: &mut S) -> Result<usize, S::Error> {
        self
          .serialize_dyn(&mut serializer)
          .map_err(|e| *e.downcast::<S::Error>().unwrap())
      }

      fn serialize_metadata(&self, _: &mut S) -> Result<Self::MetadataResolver, S::Error> {
        Ok(())
      }
    }

    impl<D> DeserializeUnsized<dyn Animal, D> for dyn DeserializeAnimal
    where
      D: Fallible + ?Sized,
    {
      unsafe fn deserialize_unsized(
        &self,
        mut deserializer: &mut D,
        mut alloc: impl FnMut(Layout) -> *mut u8,
      ) -> Result<*mut (), D::Error> {
        self
          .deserialize_dyn(&mut deserializer, &mut alloc)
          .map_err(|e| *e.downcast().unwrap())
      }

      fn deserialize_metadata(
        &self,
        mut deserializer: &mut D,
      ) -> Result<<dyn Animal as ptr_meta::Pointee>::Metadata, D::Error> {
        self
          .deserialize_dyn_metadata(&mut deserializer)
          .map_err(|e| *e.downcast().unwrap())
      }
    }

    // CheckBytes
    impl LayoutRaw for dyn DeserializeAnimal {
      fn layout_raw(
        metadata: <Self as ptr_meta::Pointee>::Metadata,
      ) -> Result<Layout, LayoutError> {
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
      rkyv_dyn::{
        self, register_impl, register_validation, DeserializeDyn, DynDeserializer, DynError,
      },
    };
    register_impl!(ArchivedDog as <dyn Animal as ArchiveUnsized>::Archived);

    impl DeserializeDyn<dyn Animal> for Archived<Dog>
    where
      Archived<Dog>: Deserialize<Dog, dyn DynDeserializer>,
    {
      unsafe fn deserialize_dyn(
        &self,
        deserializer: &mut dyn DynDeserializer,
        alloc: &mut dyn FnMut(Layout) -> *mut u8,
      ) -> Result<*mut (), DynError> {
        let result = alloc(Layout::new::<Dog>()).cast::<Dog>();
        assert!(!result.is_null());
        result.write(self.deserialize(deserializer)?);
        Ok(result as *mut ())
      }

      fn deserialize_dyn_metadata(
        &self,
        _: &mut dyn DynDeserializer,
      ) -> Result<<dyn Animal as ptr_meta::Pointee>::Metadata, DynError> {
        unsafe {
          Ok(core::mem::transmute(ptr_meta::metadata(
            core::ptr::null::<Dog>() as *const dyn Animal,
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

  impl Animal for Cat {
    fn color(&self) -> &str {
      &self.color
    }
    fn name(&self) -> &str {
      "cat"
    }
  }

  const _: () = {
    use core::alloc::Layout;

    use rspack_cacheable::__private::{
      ptr_meta,
      rkyv::{ArchiveUnsized, Archived, Deserialize},
      rkyv_dyn::{self, register_impl, DeserializeDyn, DynDeserializer, DynError},
    };
    register_impl!(ArchivedCat as <dyn Animal as ArchiveUnsized>::Archived);

    impl DeserializeDyn<dyn Animal> for Archived<Cat>
    where
      Archived<Cat>: Deserialize<Cat, dyn DynDeserializer>,
    {
      unsafe fn deserialize_dyn(
        &self,
        deserializer: &mut dyn DynDeserializer,
        alloc: &mut dyn FnMut(Layout) -> *mut u8,
      ) -> Result<*mut (), DynError> {
        let result = alloc(Layout::new::<Cat>()).cast::<Cat>();
        assert!(!result.is_null());
        result.write(self.deserialize(deserializer)?);
        Ok(result as *mut ())
      }

      fn deserialize_dyn_metadata(
        &self,
        _: &mut dyn DynDeserializer,
      ) -> Result<<dyn Animal as ptr_meta::Pointee>::Metadata, DynError> {
        unsafe {
          Ok(core::mem::transmute(ptr_meta::metadata(
            core::ptr::null::<Cat>() as *const dyn Animal,
          )))
        }
      }
    }
  };

  #[cacheable]
  struct Data {
    animal: Box<dyn Animal>,
  }

  let dog_data = Data {
    animal: Box::new(Dog {
      color: String::from("black"),
    }),
  };
  assert_eq!(dog_data.animal.name(), "dog");
  assert_eq!(dog_data.animal.color(), "black");
  let mut ctx = Context {};
  let bytes = to_bytes(&dog_data, &mut ctx).unwrap();
  let deserialize_data = from_bytes::<Data, Context>(&bytes, &mut ctx).unwrap();
  assert_eq!(deserialize_data.animal.name(), "dog");
  assert_eq!(deserialize_data.animal.color(), "black");

  let cat_data = Data {
    animal: Box::new(Cat {
      color: String::from("white"),
    }),
  };
  assert_eq!(cat_data.animal.name(), "cat");
  assert_eq!(cat_data.animal.color(), "white");
  let mut ctx = Context {};
  let bytes = to_bytes(&cat_data, &mut ctx).unwrap();
  let deserialize_data = from_bytes::<Data, Context>(&bytes, &mut ctx).unwrap();
  assert_eq!(deserialize_data.animal.name(), "cat");
  assert_eq!(deserialize_data.animal.color(), "white");
}
