//! pvec.rs

use crate::core::VoidpMut;
//use std::{ffi};

/// Pvec
pub struct Pvec {
  /// v
  pub v: Vec<VoidpMut>
}

/// trait Drop for Pvec
impl Drop for Pvec {
  /// drop
  fn drop(&mut self) {
  }
}

/// Pvec
impl Pvec {
  /// new
  pub fn new(v: Vec<VoidpMut>) -> Self {
    Pvec{v}
  }

  /// disp
  pub fn disp(&self) -> String {
    let s = self.v.iter().enumerate().map(|(i, &e)| format!("{} {:016x}",
      if self.v.len() > 4 && i % 4 == 0 { "\n" } else { "" }, e as usize));
    format!("[{}]", s.collect::<Vec<_>>().concat())
  }
}

/// trait Display for Pvec
impl std::fmt::Display for Pvec {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp())
  }
}

/// trait Debug for Pvec
impl std::fmt::Debug for Pvec {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp())
  }
}
