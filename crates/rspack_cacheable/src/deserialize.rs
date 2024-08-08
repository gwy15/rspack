use rkyv::{
  check_archived_root,
  de::{deserializers::SharedDeserializeMap, SharedDeserializeRegistry, SharedPointer},
  validation::validators::DefaultValidator,
  Archive, CheckBytes, Deserialize, Fallible,
};

#[derive(Debug)]
pub enum DeserializeError {
  /// A validation error occurred
  CheckBytesError,
  /// A shared pointer was added multiple times
  DuplicateSharedPointer,
  /// A deserialize failed occurred
  DeserializeFailed(&'static str),
}

pub struct CacheableDeserializer {
  shared: SharedDeserializeMap,
  context: *mut (),
}

impl CacheableDeserializer {
  fn new<C>(context: &mut C) -> Self {
    Self {
      shared: SharedDeserializeMap::default(),
      context: context as *mut C as *mut (),
    }
  }

  // TODO change to safe implement
  pub unsafe fn context_mut<C>(&mut self) -> &mut C {
    std::mem::transmute::<*mut (), &mut C>(self.context)
  }
}

impl Fallible for CacheableDeserializer {
  type Error = DeserializeError;
}

impl SharedDeserializeRegistry for CacheableDeserializer {
  fn get_shared_ptr(&mut self, ptr: *const u8) -> Option<&dyn SharedPointer> {
    self.shared.get_shared_ptr(ptr)
  }

  fn add_shared_ptr(
    &mut self,
    ptr: *const u8,
    shared: Box<dyn SharedPointer>,
  ) -> Result<(), Self::Error> {
    self
      .shared
      .add_shared_ptr(ptr, shared)
      .map_err(|_| DeserializeError::DuplicateSharedPointer)
  }
}

pub fn from_bytes<'a, T, C>(bytes: &'a [u8], context: &'a mut C) -> Result<T, DeserializeError>
where
  T: Archive,
  T::Archived: 'a + CheckBytes<DefaultValidator<'a>> + Deserialize<T, CacheableDeserializer>,
{
  let mut deserializer = CacheableDeserializer::new(context);
  check_archived_root::<T>(bytes)
    .map_err(|_| DeserializeError::CheckBytesError)?
    .deserialize(&mut deserializer)
}
