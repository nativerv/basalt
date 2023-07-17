
#[macro_export]
macro_rules! serial_tests {
  (
    $(
      fn $name:ident() $body:block
    ),*
  ) => {
    
    use std::sync::Mutex;

    lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

    $(
      #[test]
      fn $name() {
          let _guard = $crate::TEST_MUTEX.lock().unwrap();
          $body
      }
    )*
  }
}

// Example use serial_tests!:
// serial_tests! {
//   fn test1() {
//     some tests...
//   },

//   fn test2() {
//     some tests...
//   }
// }