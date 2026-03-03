//! space.rs

use crate::core::*;
use crate::w32a::*;
use std::{result::Result, error::Error};
use windows::{
  Win32::Graphics::Direct3D9::*
};

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
    let dif = [1.0f32, 1.0f32, 1.0f32, 1.0f32]; // r, g, b, _
    let spc = [1.0f32, 1.0f32, 1.0f32, 1.0f32]; // r, g, b, _
    let amb = [1.0f32, 1.0f32, 1.0f32, 1.0f32]; // r, g, b, _
    let pos = [0.0f32, 0.0f32, 0.0f32, 0.0f32]; // x, y, z, _
    let dir = [1.0f32, -1.0f32, 1.0f32, 1.0f32]; // [1.0, -1.0, -1.0, 1.0]
    let _ = dx.set_light(0, D3DLIGHT_DIRECTIONAL,
      &dif, &spc, &amb, &pos, &dir, 200.0f32);
    let _ = dx.draw_d3d(t);
    Ok(())
  }

  /// step
  fn step(&mut self, dx: &mut Dx9) -> Result<(), Box<dyn Error>> {
    let _ = dx.update_d3d();
    Ok(())
  }
}
