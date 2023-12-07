// use rspack_core::Bundle;
// use rspack_core::ChunkGraph;

use tracing::instrument;

use crate::Compilation;

mod code_splitter;
mod remove_parent_modules;
pub use code_splitter::DependenciesBlockIdentifier;

#[instrument(skip_all)]
pub(crate) fn build_chunk_graph(compilation: &mut Compilation) -> rspack_error::Result<()> {
  code_splitter::CodeSplitter::new(compilation).split()?;
  Ok(())
}
