use std::collections::HashMap;

pub struct DBSchema {
  tables_cache: HashMap<String, i32>,
}

impl Default for DBSchema {
  fn default() -> Self {
    Self::new()
  }
}

impl DBSchema {
  pub fn new() -> DBSchema {
    DBSchema {
      tables_cache: HashMap::new(),
    }
  }
}

