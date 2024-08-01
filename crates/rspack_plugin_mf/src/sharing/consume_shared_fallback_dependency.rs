use rspack_cacheable::{cacheable, cacheable_dyn};
use rspack_core::{
  cache::CacheContext, AsContextDependency, AsDependencyTemplate, Dependency, DependencyCategory,
  DependencyId, DependencyType, ModuleDependency,
};

#[cacheable]
#[derive(Debug, Clone)]
pub struct ConsumeSharedFallbackDependency {
  id: DependencyId,
  request: String,
}

impl ConsumeSharedFallbackDependency {
  pub fn new(request: String) -> Self {
    Self {
      id: DependencyId::new(),
      request,
    }
  }
}

#[cacheable_dyn(CacheContext)]
impl Dependency for ConsumeSharedFallbackDependency {
  fn id(&self) -> &DependencyId {
    &self.id
  }

  fn dependency_type(&self) -> &DependencyType {
    &DependencyType::ConsumeSharedFallback
  }

  fn category(&self) -> &DependencyCategory {
    &DependencyCategory::Esm
  }
}

impl ModuleDependency for ConsumeSharedFallbackDependency {
  fn request(&self) -> &str {
    &self.request
  }
}

impl AsContextDependency for ConsumeSharedFallbackDependency {}
impl AsDependencyTemplate for ConsumeSharedFallbackDependency {}
