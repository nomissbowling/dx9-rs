//! space.rs

use crate::core::*;
use crate::w32a::*;
use std::{result::Result, error::Error};

/// Fncs
pub struct Fncs {
  /// render
  pub render: fn (&mut Space, &mut Dx9, &mut TransScreen) ->
    Result<(), Box<dyn Error>>,
  /// step
  pub step: fn (&mut Space, &mut Dx9) -> Result<(), Box<dyn Error>>
}

/// Space
pub struct Space {
  /// fncs
  pub fncs: Fncs
}

/// Space
impl Space {
  /// render
  pub fn render(&mut self, dx: &mut Dx9, t: &mut TransScreen) ->
    Result<(), Box<dyn Error>> {
    (self.fncs.render)(self, dx, t)
  }

  /// step
  pub fn step(&mut self, dx: &mut Dx9) -> Result<(), Box<dyn Error>> {
    (self.fncs.step)(self, dx)
  }
}
