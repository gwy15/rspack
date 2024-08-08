use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_quote, spanned::Spanned, GenericParam, Ident, ItemImpl, ItemTrait, Type};

pub fn impl_trait(mut input: ItemTrait) -> TokenStream {
  let trait_ident = &input.ident;
  let generic_params = input.generics.params.iter().map(|p| {
    // remove default value
    let mut p = p.clone();
    if let GenericParam::Type(param) = &mut p {
      param.eq_token = None;
      param.default = None;
    }
    quote! { #p }
  });
  let generic_params = quote! { #(#generic_params),* };
  let deserialize_trait_ident =
    Ident::new(&format!("Deserialize{trait_ident}"), trait_ident.span());
  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

  input
    .supertraits
    .push(parse_quote! { rspack_cacheable::r#dyn::SerializeDyn });

  quote! {
      #input

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

            impl #impl_generics ptr_meta::Pointee for dyn #trait_ident #ty_generics #where_clause {
                type Metadata = ptr_meta::DynMetadata<Self>;
            }

            trait #deserialize_trait_ident #ty_generics: DeserializeDyn<dyn #trait_ident #ty_generics> {}
            impl #ty_generics ptr_meta::Pointee for dyn #deserialize_trait_ident #ty_generics {
                type Metadata = ptr_meta::DynMetadata<Self>;
            }

            impl<__O: DeserializeDyn<dyn #trait_ident #ty_generics>, #generic_params> #deserialize_trait_ident #ty_generics for __O {}

            impl #ty_generics TypeName for dyn #deserialize_trait_ident #ty_generics {
                fn build_type_name<F: FnMut(&str)>(mut f: F) {
                    f(core::concat!(core::module_path!(), "::"));
                    f(&core::line!().to_string());
                }
            }

            impl #ty_generics ArchiveUnsized for dyn #trait_ident #ty_generics {
                type Archived = dyn #deserialize_trait_ident #ty_generics;
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

            impl #ty_generics ArchivePointee for dyn #deserialize_trait_ident #ty_generics {
                type ArchivedMetadata = ArchivedDynMetadata<Self>;

                fn pointer_metadata(
                    archived: &Self::ArchivedMetadata,
                ) -> <Self as ptr_meta::Pointee>::Metadata {
                    archived.pointer_metadata()
                }
            }

            impl #ty_generics SerializeUnsized<CacheableSerializer> for dyn #trait_ident #ty_generics {
                fn serialize_unsized(
                    &self,
                    mut serializer: &mut CacheableSerializer
                ) -> Result<usize, SerializeError> {
                    self.serialize_dyn(&mut serializer)
                }

                fn serialize_metadata(
                    &self,
                    _: &mut CacheableSerializer
                ) -> Result<Self::MetadataResolver, SerializeError> {
                    Ok(())
                }
            }

            impl #ty_generics DeserializeUnsized<dyn #trait_ident #ty_generics, CacheableDeserializer> for dyn #deserialize_trait_ident #ty_generics {
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
                ) -> Result<<dyn #trait_ident #ty_generics as ptr_meta::Pointee>::Metadata, DeserializeError> {
                    self.deserialize_dyn_metadata(&mut deserializer)
                }
            }

            // CheckBytes
            impl #ty_generics LayoutRaw for dyn #deserialize_trait_ident #ty_generics {
                fn layout_raw(
                    metadata: <Self as ptr_meta::Pointee>::Metadata,
                ) -> Result<Layout, LayoutError> {
                    Ok(metadata.layout())
                }
            }
            impl<__C: DynContext, #generic_params> CheckBytes<__C> for dyn #deserialize_trait_ident #ty_generics {
                type Error = CheckDynError;
                #[inline]
                unsafe fn check_bytes<'a>(
                    value: *const Self,
                    context: &mut __C,
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
  }
  .into()
}

pub fn impl_impl(input: ItemImpl) -> TokenStream {
  let trait_ident = &input.trait_.as_ref().expect("should have trait ident").1;
  let target_ident = &input.self_ty;
  let archived_target_ident = match &*input.self_ty {
    Type::Path(inner) => Ident::new(
      &format!(
        "Archived{}",
        inner
          .path
          .segments
          .last()
          .expect("should have segments")
          .ident
          .to_string()
      ),
      inner.span(),
    ),
    _ => {
      panic!("cacheable_dyn unsupport this target")
    }
  };

  quote! {
      #input

      const _: () = {
          use core::alloc::Layout;

          use rspack_cacheable::__private::{
              inventory, ptr_meta,
              rkyv::{ArchiveUnsized, Archived, Deserialize},
              rkyv_dyn::{self, register_impl},
          };
          use rspack_cacheable::{r#dyn::DeserializeDyn, CacheableDeserializer, DeserializeError};

          register_impl!(#archived_target_ident as <dyn #trait_ident as ArchiveUnsized>::Archived);

          impl DeserializeDyn<dyn #trait_ident> for Archived<#target_ident>
          where
              Archived<#target_ident>: Deserialize<#target_ident, CacheableDeserializer>,
          {
              unsafe fn deserialize_dyn(
                  &self,
                  deserializer: &mut CacheableDeserializer,
                  alloc: &mut dyn FnMut(Layout) -> *mut u8,
              ) -> Result<*mut (), DeserializeError> {
                  let result = alloc(Layout::new::<#target_ident>()).cast::<#target_ident>();
                  assert!(!result.is_null());
                  result.write(self.deserialize(deserializer)?);
                  Ok(result as *mut ())
              }

              fn deserialize_dyn_metadata(
                  &self,
                  _: &mut CacheableDeserializer,
              ) -> Result<<dyn #trait_ident as ptr_meta::Pointee>::Metadata, DeserializeError> {
                  unsafe {
                      Ok(core::mem::transmute(ptr_meta::metadata(
                          core::ptr::null::<#target_ident>() as *const dyn #trait_ident,
                      )))
                  }
              }
          }
      };
  }
  .into()
}
