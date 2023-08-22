use std::io;
use std::path::Path;
use std::path::PathBuf;
use uuid::Uuid;

/// Creates a directory with random name in system's temporary directory.
/// If you need this in the future anywhere except tests, move it to a new
/// `lib::temp_file` module and move `uuid` from dev_dependencies to
/// dependencies.
pub fn temp_dir() -> std::io::Result<PathBuf> {
  let random_path = std::env::temp_dir().join(Uuid::new_v4().to_string());
  std::fs::create_dir(&random_path)?;
  Ok(random_path)
}

/// Creates a temporary directory for a closure of yours and deletes it afterwards
pub fn with_test_dir<T>(f: impl Fn(&Path) -> io::Result<T>) -> io::Result<T> {
  let tmp_dir = temp_dir().expect("test: can't create temp dir");
  let value = f(&tmp_dir);
  std::fs::remove_dir_all(tmp_dir).expect("test: can't remove");
  value
}
