use core::alloc::Layout;
use std::hash::{DefaultHasher, Hash, Hasher};

use ptr_meta::Pointee;
use rkyv::{ser::Serializer, Serialize};
use rkyv_typename::TypeName;

use crate::{CacheableDeserializer, CacheableSerializer, DeserializeError, SerializeError};

/// A trait object forked from rkyv_dyn::SerializeDyn
///
/// This trait will override some internal methods params.
/// 1. serializer from `&mut dyn DynSerializer` to `CacheableSerializer`
/// 2. return Error from `DynError` to `SerializeError`
pub trait SerializeDyn {
  /// Writes the value to the serializer and returns the position it was written to.
  fn serialize_dyn(&self, serializer: &mut CacheableSerializer) -> Result<usize, SerializeError>;

  /// Returns the type ID of the archived version of this type.
  fn archived_type_id(&self) -> u64;
}

impl<T: Serialize<CacheableSerializer>> SerializeDyn for T
where
  T::Archived: TypeName,
{
  fn serialize_dyn(&self, serializer: &mut CacheableSerializer) -> Result<usize, SerializeError> {
    serializer.serialize_value(self)
  }

  fn archived_type_id(&self) -> u64 {
    // TODO rewrite to not depend on TypeName
    let mut hasher = DefaultHasher::new();
    T::Archived::build_type_name(|s| s.hash(&mut hasher));
    hasher.finish()
  }
}

/// A trait object forked from rkyv_dyn::DeserializeDyn
///
/// This trait will override some internal methods params.
/// 1. deserializer from `&mut dyn DynDeserializer` to `CacheableDeserializer`
/// 2. return Error from `DynError` to `DeserializeError`
pub trait DeserializeDyn<T: Pointee + ?Sized> {
  /// Deserializes the given value as a trait object.
  unsafe fn deserialize_dyn(
    &self,
    deserializer: &mut CacheableDeserializer,
    alloc: &mut dyn FnMut(Layout) -> *mut u8,
  ) -> Result<*mut (), DeserializeError>;

  /// Returns the metadata for the deserialized version of this value.
  fn deserialize_dyn_metadata(
    &self,
    deserializer: &mut CacheableDeserializer,
  ) -> Result<T::Metadata, DeserializeError>;
}

/* pub mod validation;

#[cfg(feature = "vtable_cache")]
use core::sync::atomic::{AtomicU64, Ordering};
use core::{
    alloc::Layout,
    any::Any,
    hash::{Hash, Hasher},
    marker::PhantomData,
    ptr,
};
use ptr_meta::{DynMetadata, Pointee};
#[cfg(feature = "vtable_cache")]
use rkyv::with::{Atomic, With};
use rkyv::{
    from_archived,
    ser::{ScratchSpace, Serializer},
    to_archived, Archived, Fallible, Serialize,
};
pub use rkyv_dyn_derive::archive_dyn;
#[cfg(all(feature = "vtable_cache", feature = "nightly"))]
use core::intrinsics::likely;
use std::collections::{hash_map::DefaultHasher, HashMap};

#[doc(hidden)]
pub use inventory;
use rkyv_typename::TypeName;
pub use validation::{CheckDynError, DynContext};*/

fn hash_type<T: TypeName + ?Sized>() -> u64 {
  let mut hasher = DefaultHasher::new();
  T::build_type_name(|piece| piece.hash(&mut hasher));
  hasher.finish()
}

/// The archived version of `DynMetadata`.
#[cfg_attr(feature = "strict", repr(C))]
pub struct ArchivedDynMetadata<T: ?Sized> {
  type_id: Archived<u64>,
  #[cfg(feature = "vtable_cache")]
  cached_vtable: Archived<With<AtomicU64, Atomic>>,
  #[cfg(not(feature = "vtable_cache"))]
  #[allow(dead_code)]
  cached_vtable: Archived<u64>,
  phantom: PhantomData<T>,
}

impl<T: TypeName + ?Sized> ArchivedDynMetadata<T> {
  /// Creates a new `ArchivedDynMetadata` for the given type.
  ///
  /// # Safety
  ///
  /// `out` must point to a valid location for an `ArchivedDynMetadata<T>`.
  pub unsafe fn emplace(type_id: u64, out: *mut Self) {
    ptr::addr_of_mut!((*out).type_id).write(to_archived!(type_id));
    #[cfg(feature = "vtable_cache")]
    {
      let cached_vtable = ptr::addr_of_mut!((*out).cached_vtable);
      (*cached_vtable).store(0u64, Ordering::Relaxed);
    }
    #[cfg(not(feature = "vtable_cache"))]
    ptr::addr_of_mut!((*out).cached_vtable).write(to_archived!(0u64));
  }

  fn lookup_vtable(&self) -> usize {
    IMPL_REGISTRY
      .get::<T>(from_archived!(self.type_id))
      .expect("attempted to get vtable for an unregistered impl")
      .vtable
  }

  /// Gets the vtable address for this trait object. With the `vtable_cache` feature, this will
  /// store the address locally on the first lookup.
  #[cfg(feature = "vtable_cache")]
  pub fn vtable(&self) -> usize {
    let cached_vtable = self.cached_vtable.load(Ordering::Relaxed);
    if likely(cached_vtable != 0) {
      return cached_vtable as usize;
    }
    let vtable = self.lookup_vtable();
    self
      .cached_vtable
      .store(vtable as usize as u64, Ordering::Relaxed);
    vtable
  }

  /// Gets the vtable address for this trait object. With the `vtable_cache` feature, this will
  /// store the address locally on the first lookup.
  #[cfg(not(feature = "vtable_cache"))]
  pub fn vtable(&self) -> usize {
    self.lookup_vtable()
  }

  /// Gets the `DynMetadata` associated with this `ArchivedDynMetadata`.
  pub fn pointer_metadata(&self) -> DynMetadata<T> {
    unsafe { core::mem::transmute(self.vtable()) }
  }
}

#[derive(Copy, Clone)]
pub struct ImplDebugInfo {
  pub file: &'static str,
  pub line: u32,
  pub column: u32,
}

#[macro_export]
macro_rules! debug_info {
  () => {
    rkyv_dyn::ImplDebugInfo {
      file: core::file!(),
      line: core::line!(),
      column: core::column!(),
    }
  };
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct ImplData {
  pub vtable: usize,
  pub debug_info: ImplDebugInfo,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct ImplId {
  trait_id: u64,
  type_id: u64,
}

impl ImplId {
  fn new<TY: TypeName, TR: TypeName + ?Sized>() -> Self {
    Self::from_type_id::<TR>(hash_type::<TY>())
  }

  fn from_type_id<TR: TypeName + ?Sized>(type_id: u64) -> Self {
    Self {
      trait_id: hash_type::<TR>(),
      // The last bit of the type ID is set to 1 to make sure we can differentiate between
      // cached and uncached vtables when the feature is turned on
      type_id: type_id | 1,
    }
  }
}

#[doc(hidden)]
pub struct ImplEntry {
  impl_id: ImplId,
  data: ImplData,
}

impl ImplEntry {
  #[doc(hidden)]
  pub fn new<TY: TypeName + RegisteredImpl<TR>, TR: TypeName + ?Sized>() -> Self {
    Self {
      impl_id: ImplId::new::<TY, TR>(),
      data: ImplData {
        vtable: <TY as RegisteredImpl<TR>>::vtable(),
        debug_info: <TY as RegisteredImpl<TR>>::debug_info(),
      },
    }
  }
}

inventory::collect!(ImplEntry);

struct ImplRegistry {
  id_to_data: HashMap<ImplId, ImplData>,
}

impl ImplRegistry {
  fn new() -> Self {
    Self {
      id_to_data: HashMap::new(),
    }
  }

  fn add_entry(&mut self, entry: &ImplEntry) {
    let old_value = self.id_to_data.insert(entry.impl_id, entry.data);

    #[cfg(debug_assertions)]
    if let Some(old_data) = old_value {
      eprintln!("impl id conflict, a trait implementation was likely added twice (but it's possible there was a hash collision)");
      eprintln!(
        "existing impl registered at {}:{}:{}",
        old_data.debug_info.file, old_data.debug_info.line, old_data.debug_info.column
      );
      eprintln!(
        "new impl registered at {}:{}:{}",
        entry.data.debug_info.file, entry.data.debug_info.line, entry.data.debug_info.column
      );
      panic!();
    }

    debug_assert!(old_value.is_none(), "impl id conflict, a trait implementation was likely added twice (but it's possible there was a hash collision)");
  }

  fn get<T: TypeName + ?Sized>(&self, type_id: u64) -> Option<&ImplData> {
    self.id_to_data.get(&ImplId::from_type_id::<T>(type_id))
  }
}

lazy_static::lazy_static! {
    static ref IMPL_REGISTRY: ImplRegistry = {
        let mut result = ImplRegistry::new();
        for entry in inventory::iter::<ImplEntry> {
            result.add_entry(entry);
        }
        result
    };
}

/// Guarantees that an impl has been registered for the type as the given trait object.
#[doc(hidden)]
pub unsafe trait RegisteredImpl<T: ?Sized> {
  fn vtable() -> usize;
  fn debug_info() -> ImplDebugInfo;
}

#[doc(hidden)]
#[cfg(not(feature = "validation"))]
#[macro_export]
macro_rules! register_validation {
  ($type:ty as $trait:ty) => {};
}

/// Registers a new impl with the trait object system.
///
/// This is called by `#[archive_dyn]` when attached to a trait implementation. You might need to
/// call this manually if you're using generic traits and types, since each specific instance needs
/// to be individually registered.
///
/// Call it like `register_impl!(MyType as dyn MyTrait)`.
#[macro_export]
macro_rules! register_impl {
  ($type:ty as $trait:ty) => {
    const _: () = {
      use rkyv_dyn::{
        debug_info, inventory, register_validation, ImplData, ImplDebugInfo, ImplEntry,
        RegisteredImpl,
      };

      unsafe impl RegisteredImpl<$trait> for $type {
        fn vtable() -> usize {
          unsafe {
            core::mem::transmute(ptr_meta::metadata(
              core::ptr::null::<$type>() as *const $trait
            ))
          }
        }

        fn debug_info() -> ImplDebugInfo {
          debug_info!()
        }
      }

      inventory::submit! { ImplEntry::new::<$type, $trait>() }
      register_validation!($type as $trait);
    };
  };
}
