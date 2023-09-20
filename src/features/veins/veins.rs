use crate::features::configuration::Configuration;
use crate::features::veins::Vein;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;

/// Vein id: just a *newtype* from string
#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, Debug, Clone)]
pub struct VeinId(String);

impl Deref for VeinId {
  type Target = str;
  fn deref(&self) -> &'_ Self::Target {
    self.0.as_str()
  }
}

use std::cell::RefCell;
use std::rc::Rc;
type VeinsHashMap = HashMap<VeinId, Rc<RefCell<Vein>>>;
/// Veins: struct that contains all veins known to the program.
pub struct Veins(VeinsHashMap);

impl From<VeinsHashMap> for Veins {
  fn from(hash_map: VeinsHashMap) -> Self {
    Self(hash_map)
  }
}

impl Veins {
  /// Constructs `Veins` from vein entries in `Configuration`,
  /// loading them from the system storage (filesystem, etc).
  pub fn from_configuration(configuration: &Configuration) -> Self {
    #[cfg(any(unix, windows))]
    return configuration
      .veins
      .iter()
      // FIXME: handle ignored error (load vein)
      // (otherwise invalid veins will be ignored)
      .filter_map(|vein_id| {
        Vein::new_native(Path::new(&**vein_id))
          .map(|vein| (VeinId(vein_id.to_string()), Rc::new(RefCell::new(vein))))
          .ok()
      })
      .collect::<VeinsHashMap>()
      .into();

    #[cfg(not(any(unix, windows)))]
    return unimplemented!();
  }

  pub fn iter(&self) -> Iter<'_> {
    Iter {
      veins_iter: self.0.iter(),
    }
  }
}

pub struct Iter<'a> {
  veins_iter: std::collections::hash_map::Iter<'a, VeinId, Rc<RefCell<Vein>>>,
}

impl<'a> Iterator for Iter<'a> {
  type Item = (&'a VeinId, &'a Rc<RefCell<Vein>>);

  fn next(&mut self) -> Option<Self::Item> {
    self.veins_iter.next()
  }
}
