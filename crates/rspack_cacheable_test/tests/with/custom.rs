use std::sync::Arc;

use rspack_cacheable::{
  cacheable,
  with::{Custom, CustomConverter},
  DeserializeError, SerializeError,
};

#[derive(Debug, PartialEq, Eq)]
enum Filename {
  Template(String),
  Fn(fn() -> String),
}

struct Context {
  filename: Arc<Filename>,
}

#[cacheable]
#[derive(Debug, PartialEq, Eq)]
struct RuntimeModule {
  name: String,
  #[with(Custom<FilenameConverter>)]
  filename: Arc<Filename>,
}

#[cacheable]
struct FilenameConverter;

impl CustomConverter<Arc<Filename>, Context> for FilenameConverter {
  fn to(self, ctx: &mut Context) -> Result<Arc<Filename>, DeserializeError> {
    Ok(ctx.filename.clone())
  }
  fn from(_data: &Arc<Filename>, _ctx: &mut Context) -> Result<Self, SerializeError>
  where
    Self: Sized,
  {
    Ok(FilenameConverter)
  }
}

#[test]
fn test_by_custom() {
  fn filename_fn() -> String {
    String::from("abc")
  }
  let filenames: Vec<Arc<Filename>> = vec![
    Arc::new(Filename::Template(String::from("abc"))),
    Arc::new(Filename::Fn(filename_fn)),
  ];

  for filename in filenames {
    let mut ctx = Context {
      filename: filename.clone(),
    };
    let module = RuntimeModule {
      name: String::from("runtime_module"),
      filename,
    };

    let bytes = rspack_cacheable::to_bytes(&module, &mut ctx).unwrap();
    let new_module: RuntimeModule = rspack_cacheable::from_bytes(&bytes, &mut ctx).unwrap();
    assert_eq!(module, new_module);
  }
}
