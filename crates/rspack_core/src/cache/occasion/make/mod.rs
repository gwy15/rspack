mod node;

use super::super::ArcStorage;

#[derive(Debug)]
pub struct MakeOccasion {
  storage: ArcStorage,
}

const SCOPE: &'static str = "make";

impl MakeOccasion {
  pub fn new(storage: ArcStorage) -> Self {
    Self { storage }
  }

  //    pub fn
}
