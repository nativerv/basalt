use std::path::{Component, Path, PathBuf};

pub trait PathExt {
  /// Path::canonicalize, but don't check for file existance and that every middle component is a dir
  fn canonicalize_unchecked(&self) -> PathBuf;
}

impl PathExt for Path {
  /// Credit: https://github.com/rust-lang/cargo/blob/fede83ccf973457de319ba6fa0e36ead454d2e20/src/cargo/util/paths.rs#L61
  fn canonicalize_unchecked(&self) -> PathBuf {
    let mut components = self.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
      components.next();
      PathBuf::from(c.as_os_str())
    } else {
      PathBuf::new()
    };

    for component in components {
      match component {
        Component::Prefix(..) => unreachable!(),
        Component::RootDir => {
          ret.push(component.as_os_str());
        }
        Component::CurDir => {}
        Component::ParentDir => {
          ret.pop();
        }
        Component::Normal(c) => {
          ret.push(c);
        }
      }
    }
    ret
  }
}
