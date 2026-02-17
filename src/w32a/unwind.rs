//! unwind.rs

pub use std::panic::{catch_unwind, resume_unwind};
use std::cell::RefCell;
use std::any::Any;

thread_local! {
  /// to process panic! in window_proc safely after resume
  pub static UNWIND: RefCell<Option<Box<dyn Any + Send>>> = RefCell::new(None);
}

/// catch_panic macro to process panic! in window_proc safely after resume
#[macro_export]
macro_rules! catch_panic {
  ($w:expr, $s:stmt) => {
    catch_unwind(|| { // to process panic! in window_proc safely after resume
      $s
    }).unwrap_or_else(|e| {
      $w.with(|unwind| *unwind.borrow_mut() = Some(e)); // carry over to resume
      LRESULT(0)
    })
  };
}
pub use catch_panic;

/// resume_panic macro to process panic! in window_proc safely after resume
#[macro_export]
macro_rules! resume_panic {
  ($w:expr) => {
    $w.with(|unwind| {
      if let Some(e) = unwind.borrow_mut().take() { resume_unwind(e); }
    });
  };
}
pub use resume_panic;
