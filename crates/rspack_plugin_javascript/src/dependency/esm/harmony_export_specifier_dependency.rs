use rspack_cacheable::{cacheable, cacheable_dyn, with::AsPreset};
use rspack_collections::IdentifierSet;
use rspack_core::{
  AsContextDependency, AsModuleDependency, Compilation, Dependency, DependencyCategory,
  DependencyId, DependencyTemplate, DependencyType, ExportNameOrSpec, ExportsOfExportsSpec,
  ExportsSpec, HarmonyExportInitFragment, ModuleGraph, RealDependencyLocation, RuntimeSpec,
  TemplateContext, TemplateReplaceSource, UsedName,
};
use swc_core::ecma::atoms::Atom;

// Create _webpack_require__.d(__webpack_exports__, {}) for each export.
#[cacheable]
#[derive(Debug, Clone)]
pub struct HarmonyExportSpecifierDependency {
  id: DependencyId,
  range: RealDependencyLocation,
  #[with(AsPreset)]
  pub name: Atom,
  #[with(AsPreset)]
  pub value: Atom, // id
}

impl HarmonyExportSpecifierDependency {
  pub fn new(name: Atom, value: Atom, range: RealDependencyLocation) -> Self {
    Self {
      name,
      value,
      range,
      id: DependencyId::new(),
    }
  }
}

#[cacheable_dyn]
impl Dependency for HarmonyExportSpecifierDependency {
  fn id(&self) -> &DependencyId {
    &self.id
  }

  fn loc(&self) -> Option<String> {
    Some(self.range.to_string())
  }

  fn category(&self) -> &DependencyCategory {
    &DependencyCategory::Esm
  }

  fn dependency_type(&self) -> &DependencyType {
    &DependencyType::EsmExportSpecifier
  }

  fn get_exports(&self, _mg: &ModuleGraph) -> Option<ExportsSpec> {
    Some(ExportsSpec {
      exports: ExportsOfExportsSpec::Array(vec![ExportNameOrSpec::String(self.name.clone())]),
      priority: Some(1),
      can_mangle: None,
      terminal_binding: Some(true),
      from: None,
      dependencies: None,
      hide_export: None,
      exclude_exports: None,
    })
  }

  fn get_module_evaluation_side_effects_state(
    &self,
    _module_graph: &rspack_core::ModuleGraph,
    _module_chain: &mut IdentifierSet,
  ) -> rspack_core::ConnectionState {
    rspack_core::ConnectionState::Bool(false)
  }
}

impl AsModuleDependency for HarmonyExportSpecifierDependency {}

#[cacheable_dyn]
impl DependencyTemplate for HarmonyExportSpecifierDependency {
  fn apply(
    &self,
    _source: &mut TemplateReplaceSource,
    code_generatable_context: &mut TemplateContext,
  ) {
    let TemplateContext {
      init_fragments,
      compilation,
      module,
      runtime,
      concatenation_scope,
      ..
    } = code_generatable_context;
    if let Some(scope) = concatenation_scope {
      scope.register_export(self.name.clone(), self.value.to_string());
      return;
    }
    let module_graph = compilation.get_module_graph();
    let module = module_graph
      .module_by_identifier(&module.identifier())
      .expect("should have module graph module");

    let used_name = {
      let exports_info = module_graph.get_exports_info(&module.identifier());
      let used_name =
        exports_info.get_used_name(&module_graph, *runtime, UsedName::Str(self.name.clone()));
      used_name.map(|item| match item {
        UsedName::Str(name) => name,
        UsedName::Vec(vec) => {
          // vec.contains(&self.name)
          // TODO: should align webpack
          vec[0].clone()
        }
      })
    };
    if let Some(used_name) = used_name {
      init_fragments.push(Box::new(HarmonyExportInitFragment::new(
        module.get_exports_argument(),
        vec![(used_name, self.value.clone())],
      )));
    }
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

impl AsContextDependency for HarmonyExportSpecifierDependency {}
