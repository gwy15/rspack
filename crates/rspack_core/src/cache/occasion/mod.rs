mod make;

use std::sync::Arc;

pub use make::MakeOccasion;
use rspack_cacheable::{with::SkipSerializeConverter, DeserializeError};

use crate::CompilerOptions;

pub struct CacheContext {
  pub options: Arc<CompilerOptions>,
}

// impl SkipSerializeConverter for Arc<CompilerOptions> {
//   fn deserialize(context: &mut CacheContext) -> Result<Self, DeserializeError> {
//     Ok(context.options.clone())
//   }
// }
