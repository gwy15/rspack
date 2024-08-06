use rspack_cacheable::{
  cacheable, from_bytes, to_bytes,
  utils::{WithWrapper, WithWrapperRef},
  with::{AsBytes, AsBytesConverter, AsCacheable, AsMap},
};
use rustc_hash::FxHashMap as HashMap;

#[cacheable]
#[derive(Debug, PartialEq, Eq)]
pub struct Resolve {
  prefer_relative: bool,
  by_dependency: Option<ByDependency>,
}

#[cacheable(with=AsBytes)]
#[derive(Debug, PartialEq, Eq)]
pub struct ByDependency(HashMap<String, Resolve>);

impl<C> AsBytesConverter<C> for ByDependency {
  fn to_bytes(&self, context: &mut C) -> Result<Vec<u8>, rspack_cacheable::SerializeError> {
    let temp =
      WithWrapperRef::<HashMap<String, Resolve>, AsMap<AsCacheable, AsCacheable>>::new(&self.0);
    to_bytes(&temp, context)
  }
  fn from_bytes(s: &[u8], context: &mut C) -> Result<Self, rspack_cacheable::DeserializeError>
  where
    Self: Sized,
  {
    let r: WithWrapper<HashMap<String, Resolve>, AsMap<AsCacheable, AsCacheable>> =
      from_bytes(s, context)?;
    Ok(Self(r.into_inner()))
  }
}

#[test]
fn test_utils_with_wrapper() {
  let mut sub_map = HashMap::default();
  sub_map.insert(
    String::from("commonjs"),
    Resolve {
      prefer_relative: false,
      by_dependency: None,
    },
  );
  sub_map.insert(
    String::from("esm"),
    Resolve {
      prefer_relative: false,
      by_dependency: None,
    },
  );
  let resolve = Resolve {
    prefer_relative: true,
    by_dependency: Some(ByDependency(sub_map)),
  };
  let bytes = rspack_cacheable::to_bytes(&resolve, &mut ()).unwrap();
  let new_resolve: Resolve = rspack_cacheable::from_bytes(&bytes, &mut ()).unwrap();
  assert_eq!(resolve, new_resolve);
}
