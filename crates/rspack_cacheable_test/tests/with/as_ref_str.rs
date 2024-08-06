use rspack_cacheable::{
  cacheable,
  with::{AsRefStr, AsRefStrConverter},
};

#[cacheable(with=AsRefStr)]
#[derive(Debug, PartialEq, Eq)]
enum ModuleType {
  NormalModule,
  ContextModule,
}
impl AsRefStrConverter for ModuleType {
  fn as_str(&self) -> &str {
    match self {
      Self::NormalModule => "normal_module",
      Self::ContextModule => "context_module",
    }
  }
  fn from_str(s: &str) -> Self
  where
    Self: Sized,
  {
    match s {
      "normal_module" => Self::NormalModule,
      "context_module" => Self::ContextModule,
      _ => panic!("unsupport module"),
    }
  }
}

#[cacheable]
#[derive(Debug, PartialEq, Eq)]
struct Module {
  module_type: ModuleType,
}

#[test]
fn test_as_ref_string() {
  let module = Module {
    module_type: ModuleType::NormalModule,
  };

  let bytes = rspack_cacheable::to_bytes(&module, &mut ()).unwrap();
  let new_module: Module = rspack_cacheable::from_bytes(&bytes, &mut ()).unwrap();
  assert_eq!(module, new_module);
}
