//! core.rs

pub mod dx9c;
pub mod space;
pub mod prim;

pub use dx9c::*;
pub use space::*;
pub use prim::{*, xmat::*};
pub use crate::ext::{*, bridge::*, ts::*, pvec::*};
use std::{mem};

/// SZBUF 512
pub const SZBUF: usize = 512;

/// SZU64
pub const SZU64: usize = mem::size_of::<u64>();

/// SZI64
pub const SZI64: usize = mem::size_of::<i64>();

/// SZU32
pub const SZU32: usize = mem::size_of::<u32>();

/// SZI32
pub const SZI32: usize = mem::size_of::<i32>();

/// SZF32
pub const SZF32: usize = mem::size_of::<f32>();

/// trait Ptr
pub trait Ptr<T> {
  /// ptr
  fn ptr(&self) -> *const T;
  /// ptr_mut
  fn ptr_mut(&mut self) -> *mut T;
  /// void ptr
  #[inline]
  fn voidp(&self) -> Voidp {
    self.ptr() as Voidp
  }
  /// void ptr_mut
  #[inline]
  fn voidp_mut(&mut self) -> VoidpMut {
    self.ptr_mut() as VoidpMut
  }
}

/// trait Dump
pub trait Dump {
  /// dump
  fn dump(&self) -> String;
}

/// trait Disp
pub trait Disp {
  /// disp
  fn disp(&self, di: usize, df: usize) -> String;
}
