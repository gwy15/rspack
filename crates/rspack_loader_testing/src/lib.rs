#![feature(let_chains)]

use async_trait::async_trait;
use rspack_cacheable::{r#dyn::SerializeDyn, CacheableSerializer, SerializeError};
use rspack_core::{Loader, LoaderContext, RunnerContext};
use rspack_error::Result;
use rspack_loader_runner::{DisplayWithSuffix, Identifiable, Identifier};
use serde_json::json;

pub struct SimpleLoader;
#[async_trait]
impl Loader<RunnerContext> for SimpleLoader {
  async fn run(&self, loader_context: &mut LoaderContext<RunnerContext>) -> Result<()> {
    let Some(content) = loader_context.content.take() else {
      return Ok(());
    };
    let export = format!("{}-simple", content.try_into_string()?);
    loader_context.content = Some(format!("module.exports = {}", json!(export)).into());
    Ok(())
  }
}
impl Identifiable for SimpleLoader {
  fn identifier(&self) -> Identifier {
    "builtin:test-simple-loader".into()
  }
}

impl SerializeDyn for SimpleLoader {
  fn serialize_dyn(&self, _: &mut CacheableSerializer) -> Result<usize, SerializeError> {
    unreachable!()
  }

  fn archived_type_id(&self) -> u64 {
    unreachable!()
  }
}

pub struct SimpleAsyncLoader;

#[async_trait]
impl Loader<RunnerContext> for SimpleAsyncLoader {
  async fn run(&self, loader_context: &mut LoaderContext<RunnerContext>) -> Result<()> {
    let Some(content) = loader_context.content.take() else {
      return Ok(());
    };
    loader_context.content = Some(format!("{}-async-simple", content.try_into_string()?).into());
    Ok(())
  }
}
impl Identifiable for SimpleAsyncLoader {
  fn identifier(&self) -> Identifier {
    "builtin:test-simple-async-loader".into()
  }
}

impl SerializeDyn for SimpleAsyncLoader {
  fn serialize_dyn(&self, _: &mut CacheableSerializer) -> Result<usize, SerializeError> {
    unreachable!()
  }

  fn archived_type_id(&self) -> u64 {
    unreachable!()
  }
}

pub struct PitchingLoader;
#[async_trait]
impl Loader<RunnerContext> for PitchingLoader {
  async fn pitch(&self, loader_context: &mut LoaderContext<RunnerContext>) -> Result<()> {
    loader_context.content = Some(
      [
        loader_context
          .remaining_request()
          .display_with_suffix(loader_context.resource()),
        loader_context.previous_request().to_string(),
      ]
      .join(":")
      .into(),
    );
    Ok(())
  }
}
impl Identifiable for PitchingLoader {
  fn identifier(&self) -> Identifier {
    "builtin:test-pitching-loader".into()
  }
}

impl SerializeDyn for PitchingLoader {
  fn serialize_dyn(&self, _: &mut CacheableSerializer) -> Result<usize, SerializeError> {
    unreachable!()
  }

  fn archived_type_id(&self) -> u64 {
    unreachable!()
  }
}
