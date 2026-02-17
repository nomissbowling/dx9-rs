//! ext.rs

pub mod bridge;
pub mod ts;
pub mod pvec;

use crate::core::*;
use crate::w32a::utl::l;
use std::{ffi};
//use windows::Win32::Foundation::HWND;

/// isnull macro
#[macro_export]
macro_rules! isnull {
  ($r:expr) => { $r == std::ptr::null_mut() };
}
pub use isnull;

/// failed macro ($hr: HRESULT)
#[macro_export]
macro_rules! failed {
  ($hr:expr) => { $hr.is_err() };
}
pub use failed;

/*
/// cast_any macro unsafe use with #![allow(unused_unsafe)] on caller
#[macro_export]
macro_rules! cast_any {
  ($t:ty, $f:ty, $o:expr) => {
    |o:$f| -> $t { unsafe { (*(&o as *const $f as *const $t)) } }($o)
  };
}
pub use cast_any;
*/

/// cast_any macro unsafe
#[macro_export]
macro_rules! cast_any {
  ($t:ty, $f:ty, $o:expr) => { (*(&$o as *const $f as *const $t)) };
}
pub use cast_any;

/// cast_any_mut macro unsafe
#[macro_export]
macro_rules! cast_any_mut {
  ($t:ty, $f:ty, $o:expr) => { (*(&mut $o as *mut $f as *mut $t)) };
}
pub use cast_any_mut;

/// Voidp
pub type Voidp = *const ffi::c_void;

/// VoidpMut
pub type VoidpMut = *mut ffi::c_void;

/// TVoidp (from T to Voidp)
pub struct TVoidp<T>(pub T);

/// trait Into for TVoidp
impl<T> Into<Voidp> for TVoidp<T> {
  /// into
  #[inline]
  fn into(self) -> Voidp {
unsafe {
    cast_any!(Voidp, T, self.0)
}
  }
}

/// voidp (from T to Voidp)
#[inline]
pub fn voidp<T>(p: T) -> Voidp { TVoidp(p).into() }

/// TVoidpMut (from mut T to VoidpMut)
pub struct TVoidpMut<T>(pub T); // mut T

/// trait Into for TVoidpMut
impl<T> Into<VoidpMut> for TVoidpMut<T> {
  /// into
  #[inline]
  fn into(mut self) -> VoidpMut {
unsafe {
    cast_any_mut!(VoidpMut, T, self.0)
}
  }
}

/// voidp_mut (from mut T to VoidpMut)
#[inline]
pub fn voidp_mut<T>(p: T) -> VoidpMut { TVoidpMut(p).into() } // mut p: T

/*
/// FVoidp (from Voidp to T)
pub struct FVoidp(pub Voidp);

/// trait Into for FVoidp
impl<T: Clone> Into<T> for FVoidp {
  /// into
  #[inline]
  fn into(self) -> T {
unsafe {
    cast_any!(T, Voidp, self.0).clone()
}
  }
}

/// trait Into for FVoidp
impl Into<HWND> for FVoidp {
  /// into
  #[inline]
  fn into(self) -> HWND {
unsafe {
    cast_any!(HWND, Voidp, self.0).clone()
}
  }
}

/// from_voidp (from Voidp to T)
#[inline]
pub fn from_voidp<T: Clone>(p: Voidp) -> T where FVoidp: Into<T> {
  FVoidp(p).into()
}
*/

/// from_voidp (from Voidp to T)
#[inline]
pub fn from_voidp<T: Clone>(p: Voidp) -> T {
unsafe{
  cast_any!(T, Voidp, p).clone()
}
}

/*
/// FVoidpMut (from VoidpMut to mut T)
pub struct FVoidpMut(pub VoidpMut);

/// trait Into for FVoidpMut
impl<T: Clone> Into<T> for FVoidpMut {
  /// into
  #[inline]
  fn into(mut self) -> T {
unsafe {
    cast_any_mut!(T, VoidpMut, self.0).clone()
}
  }
}

/// trait Into for FVoidpMut
impl Into<HWND> for FVoidpMut {
  /// into
  #[inline]
  fn into(mut self) -> HWND {
unsafe {
    cast_any_mut!(HWND, VoidpMut, self.0).clone()
}
  }
}

/// from_voidp_mut (from VoidpMut to mut T)
#[inline]
pub fn from_voidp_mut<T: Clone>(mut p: VoidpMut) -> T where FVoidpMut: Into<T> {
  FVoidpMut(p).into()
}
*/

/// from_voidp_mut (from VoidpMut to mut T)
#[inline]
pub fn from_voidp_mut<T: Clone>(mut p: VoidpMut) -> T {
unsafe{
  cast_any_mut!(T, VoidpMut, p).clone()
}
}

/// init_log
pub fn init_log(f: &str) {
unsafe {
  initLog(&l(f)[0]);
}
}

/// out_log
pub fn out_log(f: &str, s: &str) {
unsafe {
  outLog(&l(f)[0], &l("%s")[0], &l(s)[0]); // need fmt
}
}

/// slice from *const u8 (unknown length ends with 0u8)
pub fn ref_c_u8z(p: &*const u8, sz: usize) -> &[u8] {
  let e = unsafe { std::slice::from_raw_parts(*p, sz) };
  let sz = (0..sz).into_iter().map_while(|i|
    if e[i] != b'\0' { Some(1) } else { None }).sum();
  &e[..sz]
}

/// copy from String to *mut u8 (as slice of u8s bytes)
pub fn copy_bytes(u: *mut u8, l: usize, s: String) -> usize {
  let o = unsafe { std::slice::from_raw_parts_mut(u, l) };
  for i in 0..s.len() { o[i] = s.as_bytes()[i]; }
  o[s.len()] = b'\0';
  s.len()
}

/// callback element u64 C compatible
#[unsafe(no_mangle)]
pub extern "C" fn cb_u64(u: *mut u8, l: usize, p: *mut DispMatParam) -> usize {
unsafe {
  let q = std::slice::from_raw_parts(
    (*p).m as *const u64, (*p).rows * (*p).cols); // skip (*p).w
  let d = q[(*p).cols * (*p).r + (*p).c]; // skip (*p).w
  let s = format!("{:di$}", d, di=(*p).di);
  copy_bytes(u, l, s)
}
}

/// callback element u32 C compatible
#[unsafe(no_mangle)]
pub extern "C" fn cb_u32(u: *mut u8, l: usize, p: *mut DispMatParam) -> usize {
unsafe {
  let q = std::slice::from_raw_parts(
    (*p).m as *const u32, (*p).rows * (*p).cols); // skip (*p).w
  let d = q[(*p).cols * (*p).r + (*p).c]; // skip (*p).w
  let s = format!("{:di$}", d, di=(*p).di);
  copy_bytes(u, l, s)
}
}

/// callback element i64 C compatible
#[unsafe(no_mangle)]
pub extern "C" fn cb_i64(u: *mut u8, l: usize, p: *mut DispMatParam) -> usize {
unsafe {
  let q = std::slice::from_raw_parts(
    (*p).m as *const i64, (*p).rows * (*p).cols); // skip (*p).w
  let d = q[(*p).cols * (*p).r + (*p).c]; // skip (*p).w
  let s = format!("{:di$}", d, di=(*p).di);
  copy_bytes(u, l, s)
}
}

/// callback element i32 C compatible
#[unsafe(no_mangle)]
pub extern "C" fn cb_i32(u: *mut u8, l: usize, p: *mut DispMatParam) -> usize {
unsafe {
  let q = std::slice::from_raw_parts(
    (*p).m as *const i32, (*p).rows * (*p).cols); // skip (*p).w
  let d = q[(*p).cols * (*p).r + (*p).c]; // skip (*p).w
  let s = format!("{:di$}", d, di=(*p).di);
  copy_bytes(u, l, s)
}
}

/// callback element f32 C compatible
#[unsafe(no_mangle)]
pub extern "C" fn cb_f32(u: *mut u8, l: usize, p: *mut DispMatParam) -> usize {
unsafe {
  let q = std::slice::from_raw_parts(
    (*p).m as *const f32, (*p).rows * (*p).cols); // skip (*p).w
  let f = q[(*p).cols * (*p).r + (*p).c]; // skip (*p).w
  let s = format!("{:di$.df$}", f, di=(*p).di, df=(*p).df);
  copy_bytes(u, l, s)
}
}
