use crate::features::configuration::Configuration;
use crate::features::veins::Vein;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::path::{Path, PathBuf};

/// Vein id: just a *newtype* from string
#[derive(Deserialize, Serialize, Hash, PartialEq, Eq, Debug, Clone)]
pub struct VeinId(PathBuf);

impl Deref for VeinId {
  type Target = Path;
  fn deref(&self) -> &'_ Self::Target {
    self.0.as_path()
  }
}

type VeinsHashMap = HashMap<VeinId, Vein>;
/// Veins: struct that contains all veins known to the program.
pub struct Veins(VeinsHashMap);

impl From<VeinsHashMap> for Veins {
  fn from(hash_map: VeinsHashMap) -> Self {
    Self(hash_map)
  }
}

impl Veins {
  pub fn load_from_config(configuration: &Configuration) -> Self {
    #[cfg(any(unix, windows))]
    configuration
      .veins
      .iter()
      .map(|vein_id| (VeinId(vein_id.to_path_buf()), Vein::new_native(vein_id)))
      .collect::<VeinsHashMap>()
      .into()
  }
}
