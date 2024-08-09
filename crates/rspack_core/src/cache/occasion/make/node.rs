//use rspack_cache::cacheable;

use crate::{AsyncDependenciesBlock, BoxDependency, BoxModule, ModuleIdentifier, ModuleIssuer};

//#[cacheable]
struct Node {
  issuer: ModuleIssuer,
  module: BoxModule,
  dependencies: (BoxDependency, Option<ModuleIdentifier>),
  blocks: AsyncDependenciesBlock,
}
