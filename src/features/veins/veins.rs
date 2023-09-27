use crate::features::configuration::Configuration;
use crate::features::veins::Vein;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::io;
use std::ops::Deref;
use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

/// Vein id: just a _newtype_ from allocated str
/// NOTE: uses Rc<str> instead of string for O(1) clone.
#[derive(Deserialize, Serialize, PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
pub struct VeinId(Rc<str>);

impl Deref for VeinId {
  type Target = str;
  fn deref(&self) -> &'_ Self::Target {
    &*self.0
  }
}

type MaybeVein = io::Result<Rc<RefCell<Vein>>>;
type VeinsHashMap = BTreeMap<VeinId, io::Result<Rc<RefCell<Vein>>>>;

/// Veins: struct that contains all veins known to the program.
pub struct Veins(VeinsHashMap);

impl From<VeinsHashMap> for Veins {
  fn from(hash_map: VeinsHashMap) -> Self {
    Self(hash_map)
  }
}

impl Veins {
  /// Constructs `Veins` from vein id/path/url entries in `Configuration`,
  /// loading them from the system storage (filesystem, etc).
  pub fn from_configuration(configuration: &Configuration) -> Self {
    #[cfg(any(unix, windows))]
    return configuration
      .veins
      .iter()
      .map(|vein_id| {
        (vein_id.clone(), Vein::new_native(Path::new(&**vein_id)).map(RefCell::new).map(Rc::new))
      })
      .collect::<VeinsHashMap>()
      .into()
    ;

    #[cfg(not(any(unix, windows)))]
    return unimplemented!();
  }

  /// Creates a new empty `Veins` struct
  pub fn new() -> Self {
    Self(VeinsHashMap::new())
  }

  /// Creates a new iterator over tuples of (&VeinId, &Rc<RefCell<Vein>>)
  pub fn iter(&self) -> Iter<'_> {
    Iter {
      veins_iter: self.0.iter(),
    }
  }

  /// Maybe get vein by id
  pub fn get_vein(&self, id: &VeinId) -> Option<Result<Rc<RefCell<Vein>>, &std::io::Error>> {
    self.0.get(id).map(|maybe_vein| maybe_vein.as_ref().map(Rc::clone))
  }
}

/// Iterator over tuples of (&VeinId, &Rc<RefCell<Vein>>)
pub struct Iter<'a> {
  veins_iter: std::collections::btree_map::Iter<'a, VeinId, MaybeVein>,
}

impl<'a> Iterator for Iter<'a> {
  type Item = (&'a VeinId, &'a MaybeVein);

  fn next(&mut self) -> Option<Self::Item> {
    self.veins_iter.next()
  }
}
impl<'a> DoubleEndedIterator for Iter<'a> {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.veins_iter.next_back()
  }
}
