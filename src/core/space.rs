//! space.rs

use crate::core::*;
use crate::w32a::*;
use std::{result::Result, error::Error};

/// trait TSpace
pub trait TSpace {
  /// setup
  fn setup(&mut self, dx: &mut Dx9) -> Result<(), Box<dyn Error>>;
  /// dispose
  fn dispose(&mut self, dx: &mut Dx9) -> Result<(), Box<dyn Error>>;
  /// render
  fn render(&mut self, dx: &mut Dx9, t: &mut TransScreen) ->
    Result<(), Box<dyn Error>>;
  /// step
  fn step(&mut self, dx: &mut Dx9) -> Result<(), Box<dyn Error>>;
}

/// Space
pub struct Space {
  /// obj
  pub obj: Vec<u64>
}

/// Space
impl TSpace for Space {
  /// setup
  fn setup(&mut self, _dx: &mut Dx9) -> Result<(), Box<dyn Error>> {
    Ok(())
  }

  /// dispose
  fn dispose(&mut self, _dx: &mut Dx9) -> Result<(), Box<dyn Error>> {
    Ok(())
  }

  /// render
  fn render(&mut self, dx: &mut Dx9, t: &mut TransScreen) ->
    Result<(), Box<dyn Error>> {
    let _ = dx.draw_d3d(t);
    Ok(())
  }

  /// step
  fn step(&mut self, dx: &mut Dx9) -> Result<(), Box<dyn Error>> {
    let _ = dx.update_d3d();
    Ok(())
  }
}
