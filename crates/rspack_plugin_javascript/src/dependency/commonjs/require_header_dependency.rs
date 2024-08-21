use std::sync::Arc;

use rspack_cacheable::{cacheable, cacheable_dyn};
use rspack_core::{AsContextDependency, AsModuleDependency, Compilation, Dependency, RuntimeSpec};
use rspack_core::{DependencyId, DependencyLocation};
use rspack_core::{DependencyTemplate, RuntimeGlobals, TemplateContext};
use swc_core::common::SourceMap;

#[cacheable]
#[derive(Debug, Clone)]
pub struct RequireHeaderDependency {
  id: DependencyId,
  loc: DependencyLocation,
}

impl RequireHeaderDependency {
  pub fn new(start: u32, end: u32, source: Option<Arc<SourceMap>>) -> Self {
    let loc = DependencyLocation::new(start, end, source);
    Self {
      id: DependencyId::new(),
      loc,
    }
  }
}

#[cacheable_dyn]
impl Dependency for RequireHeaderDependency {
  fn id(&self) -> &DependencyId {
    &self.id
  }
}

impl AsModuleDependency for RequireHeaderDependency {}
impl AsContextDependency for RequireHeaderDependency {}

#[cacheable_dyn]
impl DependencyTemplate for RequireHeaderDependency {
  fn apply(
    &self,
    source: &mut rspack_core::TemplateReplaceSource,
    code_generatable_context: &mut rspack_core::TemplateContext,
  ) {
    let TemplateContext {
      runtime_requirements,
      ..
    } = code_generatable_context;
    runtime_requirements.insert(RuntimeGlobals::REQUIRE);
    source.replace(
      self.loc.start(),
      self.loc.end() - 1,
      RuntimeGlobals::REQUIRE.name(),
      None,
    );
  }

  fn dependency_id(&self) -> Option<DependencyId> {
    Some(self.id)
  }

  fn update_hash(
    &self,
    _hasher: &mut dyn std::hash::Hasher,
    _compilation: &Compilation,
    _runtime: Option<&RuntimeSpec>,
  ) {
  }
}
