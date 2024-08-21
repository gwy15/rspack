use rspack_cacheable::SerializeError;
use rspack_cacheable::__private::rkyv::{
  out_field, with::With, Archive, Archived, Resolver, Serialize,
};
use rspack_cacheable::{
  cacheable,
  with::{AsTuple2, AsVec},
  CacheableSerializer,
};
use rspack_collections::IdentifierSet;
use rustc_hash::FxHashSet as HashSet;

use crate::{BuildDependency, DependencyId, FileCounter};

#[cacheable]
pub struct Meta {
  #[with(AsVec<AsTuple2>)]
  pub make_failed_dependencies: HashSet<BuildDependency>,
  #[with(AsVec)]
  pub make_failed_module: IdentifierSet,
  #[with(AsVec)]
  pub entry_dependencies: HashSet<DependencyId>,
  pub file_dependencies: FileCounter,
  pub context_dependencies: FileCounter,
  pub missing_dependencies: FileCounter,
  pub build_dependencies: FileCounter,
}

pub struct MetaRef<'a> {
  pub make_failed_dependencies: &'a HashSet<BuildDependency>,
  pub make_failed_module: &'a IdentifierSet,
  pub entry_dependencies: &'a HashSet<DependencyId>,
  pub file_dependencies: &'a FileCounter,
  pub context_dependencies: &'a FileCounter,
  pub missing_dependencies: &'a FileCounter,
  pub build_dependencies: &'a FileCounter,
}

impl<'a> Archive for MetaRef<'a> {
  type Archived = Archived<Meta>;
  type Resolver = Resolver<Meta>;

  unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
    let (fp, fo) = out_field!(out.make_failed_dependencies);
    Archive::resolve(
      With::<_, AsVec<AsTuple2>>::cast(self.make_failed_dependencies),
      pos + fp,
      resolver.make_failed_dependencies,
      fo,
    );
    let (fp, fo) = out_field!(out.make_failed_module);
    Archive::resolve(
      With::<_, AsVec>::cast(self.make_failed_module),
      pos + fp,
      resolver.make_failed_module,
      fo,
    );
    let (fp, fo) = out_field!(out.entry_dependencies);
    Archive::resolve(
      With::<_, AsVec>::cast(self.entry_dependencies),
      pos + fp,
      resolver.entry_dependencies,
      fo,
    );
    let (fp, fo) = out_field!(out.file_dependencies);
    Archive::resolve(
      self.file_dependencies,
      pos + fp,
      resolver.file_dependencies,
      fo,
    );
    let (fp, fo) = out_field!(out.context_dependencies);
    Archive::resolve(
      self.context_dependencies,
      pos + fp,
      resolver.context_dependencies,
      fo,
    );
    let (fp, fo) = out_field!(out.missing_dependencies);
    Archive::resolve(
      self.missing_dependencies,
      pos + fp,
      resolver.missing_dependencies,
      fo,
    );
    let (fp, fo) = out_field!(out.build_dependencies);
    Archive::resolve(
      self.build_dependencies,
      pos + fp,
      resolver.build_dependencies,
      fo,
    );
  }
}

impl<'a> Serialize<CacheableSerializer> for MetaRef<'a>
// where
//  With<HashSet<BuildDependency>, AsVec<AsTuple2>>: Serialize<CacheableSerializer>,
//  With<IdentifierSet, AsVec>: Serialize<CacheableSerializer>,
//  With<HashSet<DependencyId>, AsVec>: Serialize<CacheableSerializer>,
//  FileCounter: Serialize<CacheableSerializer>,
{
  #[inline]
  fn serialize(
    &self,
    serializer: &mut CacheableSerializer,
  ) -> ::core::result::Result<Self::Resolver, SerializeError> {
    Ok(MetaResolver {
      make_failed_dependencies: Serialize::<CacheableSerializer>::serialize(
        With::<_, AsVec<AsTuple2>>::cast(self.make_failed_dependencies),
        serializer,
      )?,
      make_failed_module: Serialize::<CacheableSerializer>::serialize(
        With::<_, AsVec>::cast(self.make_failed_module),
        serializer,
      )?,
      entry_dependencies: Serialize::<CacheableSerializer>::serialize(
        With::<_, AsVec>::cast(self.entry_dependencies),
        serializer,
      )?,
      file_dependencies: Serialize::<CacheableSerializer>::serialize(
        self.file_dependencies,
        serializer,
      )?,
      context_dependencies: Serialize::<CacheableSerializer>::serialize(
        self.context_dependencies,
        serializer,
      )?,
      missing_dependencies: Serialize::<CacheableSerializer>::serialize(
        self.missing_dependencies,
        serializer,
      )?,
      build_dependencies: Serialize::<CacheableSerializer>::serialize(
        self.build_dependencies,
        serializer,
      )?,
    })
  }
}
