use napi_derive::napi;
use rspack_core::{Optimization, PluginExt, SideEffectOption};
use rspack_error::internal_error;
use rspack_ids::{DeterministicModuleIdsPlugin, NamedModuleIdsPlugin};
use rspack_plugin_split_chunks::SplitChunksPlugin;
use serde::Deserialize;

use crate::{RawOptionsApply, RawSplitChunksOptions};

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[napi(object)]
pub struct RawOptimizationOptions {
  pub split_chunks: Option<RawSplitChunksOptions>,
  pub module_ids: String,
  pub remove_available_modules: bool,
  pub side_effects: String,
}

impl RawOptionsApply for RawOptimizationOptions {
  type Options = Optimization;

  fn apply(
    self,
    plugins: &mut Vec<Box<dyn rspack_core::Plugin>>,
  ) -> Result<Self::Options, rspack_error::Error> {
    if let Some(options) = self.split_chunks {
      let split_chunks_plugin = SplitChunksPlugin::new(options.into()).boxed();
      plugins.push(split_chunks_plugin);
    }
    let module_ids_plugin = match self.module_ids.as_ref() {
      "named" => NamedModuleIdsPlugin::default().boxed(),
      "deterministic" => DeterministicModuleIdsPlugin::default().boxed(),
      _ => {
        return Err(internal_error!(
          "'module_ids' should be 'named' or 'deterministic'."
        ))
      }
    };
    plugins.push(module_ids_plugin);
    Ok(Optimization {
      remove_available_modules: self.remove_available_modules,
      side_effects: SideEffectOption::from(self.side_effects.as_str()),
    })
  }
}
