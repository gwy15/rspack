use json::JsonValue;
use rspack_cacheable::{cacheable, cacheable_dyn, with::AsString};
use rspack_core::{
  AsContextDependency, AsModuleDependency, Dependency, DependencyId, DependencyTemplate,
  ExportNameOrSpec, ExportSpec, ExportsOfExportsSpec, ExportsSpec, ModuleGraph, TemplateContext,
  TemplateReplaceSource,
};
pub struct JsonExportsDependency {
  id: DependencyId,
  data: JsonValue,
}
#[automatically_derived]
impl ::core::fmt::Debug for JsonExportsDependency {
  #[inline]
  fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
    ::core::fmt::Formatter::debug_struct_field2_finish(
      f,
      "JsonExportsDependency",
      "id",
      &self.id,
      "data",
      &&self.data,
    )
  }
}
#[automatically_derived]
impl ::core::clone::Clone for JsonExportsDependency {
  #[inline]
  fn clone(&self) -> JsonExportsDependency {
    JsonExportsDependency {
      id: ::core::clone::Clone::clone(&self.id),
      data: ::core::clone::Clone::clone(&self.data),
    }
  }
}

#[repr(C)]
pub struct ArchivedJsonExportsDependency
where
  DependencyId: rspack_cacheable::__private::rkyv::Archive,
  rspack_cacheable::__private::rkyv::with::With<JsonValue, AsString>:
    rspack_cacheable::__private::rkyv::Archive,
{
  ///The archived counterpart of [`JsonExportsDependency::id`]
  id: rspack_cacheable::__private::rkyv::Archived<DependencyId>,
  ///The archived counterpart of [`JsonExportsDependency::data`]
  data: rspack_cacheable::__private::rkyv::Archived<
    rspack_cacheable::__private::rkyv::with::With<JsonValue, AsString>,
  >,
}
#[allow(unused_results)]
const _: () = {
  use ::core::{convert::Infallible, marker::PhantomData};
  use rspack_cacheable::__private::rkyv::bytecheck::{
    CheckBytes, EnumCheckError, ErrorBox, StructCheckError, TupleStructCheckError,
  };
  #[automatically_derived]
  impl<__C: ?Sized> CheckBytes<__C> for ArchivedJsonExportsDependency
  where
    DependencyId: rspack_cacheable::__private::rkyv::Archive,
    rspack_cacheable::__private::rkyv::with::With<JsonValue, AsString>:
      rspack_cacheable::__private::rkyv::Archive,
    rspack_cacheable::__private::rkyv::Archived<DependencyId>: CheckBytes<__C>,
    rspack_cacheable::__private::rkyv::Archived<
      rspack_cacheable::__private::rkyv::with::With<JsonValue, AsString>,
    >: CheckBytes<__C>,
  {
    type Error = StructCheckError;
    unsafe fn check_bytes<'__bytecheck>(
      value: *const Self,
      context: &mut __C,
    ) -> ::core::result::Result<&'__bytecheck Self, StructCheckError> {
      let bytes = value.cast::<u8>();
      <rspack_cacheable::__private::rkyv::Archived<DependencyId> as CheckBytes<__C>>::check_bytes(
        &raw const (*value).id,
        context,
      )
      .map_err(|e| StructCheckError {
        field_name: "id",
        inner: ErrorBox::new(e),
      })?;
      <rspack_cacheable::__private::rkyv::Archived<
        rspack_cacheable::__private::rkyv::with::With<JsonValue, AsString>,
      > as CheckBytes<__C>>::check_bytes(&raw const (*value).data, context)
      .map_err(|e| StructCheckError {
        field_name: "data",
        inner: ErrorBox::new(e),
      })?;
      Ok(&*value)
    }
  }
};
#[automatically_derived]
///The resolver for an archived [`JsonExportsDependency`]
pub struct JsonExportsDependencyResolver
where
  DependencyId: rspack_cacheable::__private::rkyv::Archive,
  rspack_cacheable::__private::rkyv::with::With<JsonValue, AsString>:
    rspack_cacheable::__private::rkyv::Archive,
{
  id: rspack_cacheable::__private::rkyv::Resolver<DependencyId>,
  data: rspack_cacheable::__private::rkyv::Resolver<
    rspack_cacheable::__private::rkyv::with::With<JsonValue, AsString>,
  >,
}
#[automatically_derived]
const _: () = {
  use ::core::marker::PhantomData;
  use rspack_cacheable::__private::rkyv::{out_field, Archive, Archived};
  impl Archive for JsonExportsDependency
  where
    DependencyId: rspack_cacheable::__private::rkyv::Archive,
    rspack_cacheable::__private::rkyv::with::With<JsonValue, AsString>:
      rspack_cacheable::__private::rkyv::Archive,
  {
    type Archived = ArchivedJsonExportsDependency;
    type Resolver = JsonExportsDependencyResolver;
    #[allow(clippy::unit_arg)]
    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
      let (fp, fo) = {
        #[allow(unused_unsafe)]
        unsafe {
          let fo = &raw mut (*out).id;
          (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
        }
      };
      rspack_cacheable::__private::rkyv::Archive::resolve((&self.id), pos + fp, resolver.id, fo);
      let (fp, fo) = {
        #[allow(unused_unsafe)]
        unsafe {
          let fo = &raw mut (*out).data;
          (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
        }
      };
      rspack_cacheable::__private::rkyv::Archive::resolve(
        rspack_cacheable::__private::rkyv::with::With::<_, AsString>::cast((&self.data)),
        pos + fp,
        resolver.data,
        fo,
      );
    }
  }
};
#[automatically_derived]
const _: () =
  {
    use rspack_cacheable::__private::rkyv::{Archive, Archived, Deserialize, Fallible};
    impl<__D: Fallible + ?Sized> Deserialize<JsonExportsDependency, __D>
      for Archived<JsonExportsDependency>
    where
      DependencyId: Archive,
      Archived<DependencyId>: Deserialize<DependencyId, __D>,
      rspack_cacheable::__private::rkyv::with::With<JsonValue, AsString>: Archive,
      Archived<rspack_cacheable::__private::rkyv::with::With<JsonValue, AsString>>:
        Deserialize<rspack_cacheable::__private::rkyv::with::With<JsonValue, AsString>, __D>,
    {
      #[inline]
      fn deserialize(
        &self,
        deserializer: &mut __D,
      ) -> ::core::result::Result<JsonExportsDependency, __D::Error> {
        Ok(JsonExportsDependency {
          id: Deserialize::<DependencyId, __D>::deserialize(&self.id, deserializer)?,
          data: Deserialize::<
            rspack_cacheable::__private::rkyv::with::With<JsonValue, AsString>,
            __D,
          >::deserialize(&self.data, deserializer)?
          .into_inner(),
        })
      }
    }
  };
#[automatically_derived]
const _: () = {
  use rspack_cacheable::__private::rkyv::{Archive, Fallible, Serialize};
  impl<__S: Fallible + ?Sized> Serialize<__S> for JsonExportsDependency
  where
    DependencyId: Serialize<__S>,
    rspack_cacheable::__private::rkyv::with::With<JsonValue, AsString>: Serialize<__S>,
  {
    #[inline]
    fn serialize(
      &self,
      serializer: &mut __S,
    ) -> ::core::result::Result<Self::Resolver, __S::Error> {
      Ok(JsonExportsDependencyResolver {
        id: Serialize::<__S>::serialize(&self.id, serializer)?,
        data: Serialize::<__S>::serialize(
          rspack_cacheable::__private::rkyv::with::With::<_, AsString>::cast(&self.data),
          serializer,
        )?,
      })
    }
  }
};
impl rspack_cacheable::__private::rkyv_typename::TypeName for ArchivedJsonExportsDependency {
  fn build_type_name<F: FnMut(&str)>(mut f: F) {
    f("rspack_plugin_json::json_exports_dependency::");
    f(&9u32.to_string());
  }
}
impl JsonExportsDependency {
  pub fn new(data: JsonValue) -> Self {
    Self {
      data,
      id: DependencyId::new(),
    }
  }
}
impl Dependency for JsonExportsDependency {
  fn id(&self) -> &rspack_core::DependencyId {
    &self.id
  }
  fn get_exports(&self, _mg: &ModuleGraph) -> Option<ExportsSpec> {
    Some(ExportsSpec {
      exports: get_exports_from_data(&self.data).unwrap_or(ExportsOfExportsSpec::Null),
      ..Default::default()
    })
  }
}
const _: () = {
  use core::alloc::Layout;

  use rspack_cacheable::__private::{
    inventory, ptr_meta,
    rkyv::{ArchiveUnsized, Deserialize},
    rkyv_dyn::{self, register_impl},
  };
  use rspack_cacheable::{r#dyn::DeserializeDyn, CacheableDeserializer, DeserializeError};
  const _: () = {
    use rkyv_dyn::{
      debug_info, inventory, register_validation, ImplData, ImplDebugInfo, ImplEntry,
      RegisteredImpl,
    };
    unsafe impl RegisteredImpl<<dyn Dependency as ArchiveUnsized>::Archived>
      for ArchivedJsonExportsDependency
    {
      fn vtable() -> usize {
        unsafe {
          core::mem::transmute(ptr_meta::metadata(
            core::ptr::null::<ArchivedJsonExportsDependency>()
              as *const <dyn Dependency as ArchiveUnsized>::Archived,
          ))
        }
      }
      fn debug_info() -> ImplDebugInfo {
        rkyv_dyn::ImplDebugInfo {
          file: "crates/rspack_plugin_json/src/json_exports_dependency.rs",
          line: 26u32,
          column: 1u32,
        }
      }
    }
    #[allow(non_upper_case_globals)]
    extern "C" fn __init10538706908251829851() {
      inventory::submit({
        ImplEntry::new::<ArchivedJsonExportsDependency, <dyn Dependency as ArchiveUnsized>::Archived>(
        )
      });
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = "__DATA,__mod_init_func"]
    static __init10538706908251829851___rust_ctor___ctor: unsafe extern "C" fn() = {
      unsafe extern "C" fn __init10538706908251829851___rust_ctor___ctor() {
        __init10538706908251829851()
      }
      __init10538706908251829851___rust_ctor___ctor
    };
    use rkyv_dyn::validation::{CheckBytesEntry, IsCheckBytesDyn, NotCheckBytesDyn};
    #[allow(non_upper_case_globals)]
    extern "C" fn __init9351837694764447463() {
      inventory::submit({
        CheckBytesEntry::new::<
          ArchivedJsonExportsDependency,
          <dyn Dependency as ArchiveUnsized>::Archived,
        >(IsCheckBytesDyn::<ArchivedJsonExportsDependency>::CHECK_BYTES_DYN)
      });
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = "__DATA,__mod_init_func"]
    static __init9351837694764447463___rust_ctor___ctor: unsafe extern "C" fn() = {
      unsafe extern "C" fn __init9351837694764447463___rust_ctor___ctor() {
        __init9351837694764447463()
      }
      __init9351837694764447463___rust_ctor___ctor
    };
  };
  impl DeserializeDyn<dyn Dependency> for ArchivedJsonExportsDependency
  where
    ArchivedJsonExportsDependency: Deserialize<JsonExportsDependency, CacheableDeserializer>,
  {
    unsafe fn deserialize_dyn(
      &self,
      deserializer: &mut CacheableDeserializer,
      alloc: &mut dyn FnMut(Layout) -> *mut u8,
    ) -> Result<*mut (), DeserializeError> {
      let result = alloc(Layout::new::<JsonExportsDependency>()).cast::<JsonExportsDependency>();
      if !!result.is_null() {
        ::core::panicking::panic("assertion failed: !result.is_null()")
      }
      result.write(self.deserialize(deserializer)?);
      Ok(result as *mut ())
    }
    fn deserialize_dyn_metadata(
      &self,
      _: &mut CacheableDeserializer,
    ) -> Result<<dyn Dependency as ptr_meta::Pointee>::Metadata, DeserializeError> {
      unsafe {
        Ok(core::mem::transmute(ptr_meta::metadata(
          core::ptr::null::<JsonExportsDependency>() as *const dyn Dependency,
        )))
      }
    }
  }
};
impl AsModuleDependency for JsonExportsDependency {}
impl AsContextDependency for JsonExportsDependency {}
impl DependencyTemplate for JsonExportsDependency {
  fn apply(
    &self,
    _source: &mut TemplateReplaceSource,
    _code_generatable_context: &mut TemplateContext,
  ) {
  }
  fn dependency_id(&self) -> Option<DependencyId> {
    Some(self.id)
  }
}
const _: () = {
  use core::alloc::Layout;

  use rspack_cacheable::__private::{
    inventory, ptr_meta,
    rkyv::{ArchiveUnsized, Deserialize},
    rkyv_dyn::{self, register_impl},
  };
  use rspack_cacheable::{r#dyn::DeserializeDyn, CacheableDeserializer, DeserializeError};
  const _: () = {
    use rkyv_dyn::{
      debug_info, inventory, register_validation, ImplData, ImplDebugInfo, ImplEntry,
      RegisteredImpl,
    };
    unsafe impl RegisteredImpl<<dyn DependencyTemplate as ArchiveUnsized>::Archived>
      for ArchivedJsonExportsDependency
    {
      fn vtable() -> usize {
        unsafe {
          core::mem::transmute(ptr_meta::metadata(
            core::ptr::null::<ArchivedJsonExportsDependency>()
              as *const <dyn DependencyTemplate as ArchiveUnsized>::Archived,
          ))
        }
      }
      fn debug_info() -> ImplDebugInfo {
        rkyv_dyn::ImplDebugInfo {
          file: "crates/rspack_plugin_json/src/json_exports_dependency.rs",
          line: 43u32,
          column: 1u32,
        }
      }
    }
    #[allow(non_upper_case_globals)]
    extern "C" fn __init9374351269463858488() {
      inventory::submit({
        ImplEntry::new::<
          ArchivedJsonExportsDependency,
          <dyn DependencyTemplate as ArchiveUnsized>::Archived,
        >()
      });
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = "__DATA,__mod_init_func"]
    static __init9374351269463858488___rust_ctor___ctor: unsafe extern "C" fn() = {
      unsafe extern "C" fn __init9374351269463858488___rust_ctor___ctor() {
        __init9374351269463858488()
      }
      __init9374351269463858488___rust_ctor___ctor
    };
    use rkyv_dyn::validation::{CheckBytesEntry, IsCheckBytesDyn, NotCheckBytesDyn};
    #[allow(non_upper_case_globals)]
    extern "C" fn __init7588711279538115581() {
      inventory::submit({
        CheckBytesEntry::new::<
          ArchivedJsonExportsDependency,
          <dyn DependencyTemplate as ArchiveUnsized>::Archived,
        >(IsCheckBytesDyn::<ArchivedJsonExportsDependency>::CHECK_BYTES_DYN)
      });
    }
    #[used]
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    #[link_section = "__DATA,__mod_init_func"]
    static __init7588711279538115581___rust_ctor___ctor: unsafe extern "C" fn() = {
      unsafe extern "C" fn __init7588711279538115581___rust_ctor___ctor() {
        __init7588711279538115581()
      }
      __init7588711279538115581___rust_ctor___ctor
    };
  };
  impl DeserializeDyn<dyn DependencyTemplate> for ArchivedJsonExportsDependency
  where
    ArchivedJsonExportsDependency: Deserialize<JsonExportsDependency, CacheableDeserializer>,
  {
    unsafe fn deserialize_dyn(
      &self,
      deserializer: &mut CacheableDeserializer,
      alloc: &mut dyn FnMut(Layout) -> *mut u8,
    ) -> Result<*mut (), DeserializeError> {
      let result = alloc(Layout::new::<JsonExportsDependency>()).cast::<JsonExportsDependency>();
      if !!result.is_null() {
        ::core::panicking::panic("assertion failed: !result.is_null()")
      }
      result.write(self.deserialize(deserializer)?);
      Ok(result as *mut ())
    }
    fn deserialize_dyn_metadata(
      &self,
      _: &mut CacheableDeserializer,
    ) -> Result<<dyn DependencyTemplate as ptr_meta::Pointee>::Metadata, DeserializeError> {
      unsafe {
        Ok(core::mem::transmute(ptr_meta::metadata(
          core::ptr::null::<JsonExportsDependency>() as *const dyn DependencyTemplate,
        )))
      }
    }
  }
};
fn get_exports_from_data(data: &JsonValue) -> Option<ExportsOfExportsSpec> {
  let ret = match data {
    JsonValue::Null
    | JsonValue::Short(_)
    | JsonValue::String(_)
    | JsonValue::Number(_)
    | JsonValue::Boolean(_) => {
      return None;
    }
    JsonValue::Object(obj) => ExportsOfExportsSpec::Array(
      obj
        .iter()
        .map(|(k, v)| {
          ExportNameOrSpec::ExportSpec(ExportSpec {
            name: k.into(),
            can_mangle: Some(true),
            exports: get_exports_from_data(v).map(|item| match item {
              ExportsOfExportsSpec::True => {
                ::core::panicking::panic("internal error: entered unreachable code")
              }
              ExportsOfExportsSpec::Null => {
                ::core::panicking::panic("internal error: entered unreachable code")
              }
              ExportsOfExportsSpec::Array(arr) => arr,
            }),
            ..Default::default()
          })
        })
        .collect::<Vec<_>>(),
    ),
    JsonValue::Array(arr) => {
      if arr.len() > 100 {
        return None;
      }
      ExportsOfExportsSpec::Array(
        arr
          .iter()
          .enumerate()
          .map(|(i, item)| {
            ExportNameOrSpec::ExportSpec(ExportSpec {
              name: {
                let res = ::alloc::fmt::format(format_args!("{0}", i));
                res
              }
              .into(),
              can_mangle: Some(true),
              exports: get_exports_from_data(item).map(|item| match item {
                ExportsOfExportsSpec::True | ExportsOfExportsSpec::Null => {
                  ::core::panicking::panic("internal error: entered unreachable code")
                }
                ExportsOfExportsSpec::Array(arr) => arr,
              }),
              ..Default::default()
            })
          })
          .collect::<Vec<_>>(),
      )
    }
  };
  Some(ret)
}
