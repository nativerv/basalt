use crate::features::configuration::Configuration;
use crate::features::veins::Vein;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io;
use std::ops::Deref;
use std::path::Path;

/// Vein id: just a *newtype* from string
#[derive(Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
pub struct VeinId(Rc<str>);

impl Deref for VeinId {
  type Target = str;
  fn deref(&self) -> &'_ Self::Target {
    &*self.0
  }
}

use std::cell::RefCell;
use std::rc::Rc;
type VeinsHashMap = BTreeMap<VeinId, Rc<RefCell<Vein>>>;
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
  pub fn from_configuration(configuration: &Configuration) -> io::Result<Self> {
    #[cfg(any(unix, windows))]
    return configuration
      .veins
      .iter()
      .map(|vein_id| {
        Vein::new_native(Path::new(&**vein_id))
          .map(|vein| (vein_id.clone(), Rc::new(RefCell::new(vein))))
      })
      .collect::<io::Result<VeinsHashMap>>()
      .map(Into::into)
      .into();

    #[cfg(not(any(unix, windows)))]
    return unimplemented!();
  }

  pub fn iter(&self) -> Iter<'_> {
    Iter {
      veins_iter: self.0.iter(),
    }
  }

  pub fn get_vein(&self, id: &VeinId) -> Option<Rc<RefCell<Vein>>> {
    self.0.get(id).map(Rc::clone)
  }
}

pub struct Iter<'a> {
  veins_iter: std::collections::btree_map::Iter<'a, VeinId, Rc<RefCell<Vein>>>,
}

impl<'a> Iterator for Iter<'a> {
  type Item = (&'a VeinId, &'a Rc<RefCell<Vein>>);

  fn next(&mut self) -> Option<Self::Item> {
    self.veins_iter.next()
  }
}
impl<'a> DoubleEndedIterator for Iter<'a> {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.veins_iter.next_back()
  }
}
