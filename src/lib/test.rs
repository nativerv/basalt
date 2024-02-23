use std::io;
use std::path::Path;
use crate::lib::temp_file;

/// Creates a temporary directory for a closure of yours and deletes it afterwards
pub fn with_test_dir<T>(f: impl Fn(&Path) -> io::Result<T>) -> io::Result<T> {
  let tmp_dir = temp_file::temp_dir().expect("test: can't create temp dir");
  let value = f(&tmp_dir);
  std::fs::remove_dir_all(tmp_dir).expect("test: can't remove");
  value
}
