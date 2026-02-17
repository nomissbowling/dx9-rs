//! prim.rs

pub mod xmat;

use crate::core::*;
// use crate::w32a::*;
use std::{result::Result, error::Error};
use std::{ffi};

/// D3DX_PI
pub const D3DX_PI: f32 = std::f32::consts::PI;

/// rad
#[inline]
pub fn rad(a: f32) -> f32 {
  a * D3DX_PI / 180.0f32 // instead of unsafe { D3DXToRadian(a) }
}

/// normalize_deg
pub fn normalize_deg(a: f32) -> f32 {
  let p = 180.0f32;
  let p2 = 2.0f32 * p;
  let mut a = a;
  while a > p { a -= p2 }
  while a < -p { a += p2 }
  a
}

/// normalize_rad
pub fn normalize_rad(th: f32) -> f32 {
  let pi = std::f32::consts::PI;
  let pi2 = 2.0f32 * pi;
  let mut th = th;
  while th > pi { th -= pi2 }
  while th < -pi { th += pi2 }
  th
}

/// prec_eq
#[inline]
pub fn prec_eq(a: f32, b: f32, p: f32) -> bool {
  prec_diff(a, b, p) == 0.0f32
}

/// prec_diff
pub fn prec_diff(a: f32, b: f32, p: f32) -> f32 {
  if a == b { 0.0 }
  else {
    let f = a - b;
    if f > 0.0 { if p > f { 0.0 } else { f } }
    else { if p > -f { 0.0 } else { f } }
  }
}

/// toarray macro
#[macro_export]
macro_rules! toarray {
  ($o:expr) => { $o.collect::<Vec<_>>().try_into().expect("toarray") };
}
pub use toarray;

/// XV2ULL
#[derive(Clone)]
pub struct XV2ULL (pub [ffi::c_ulonglong; 2]);

/// trait Ptr for XV2ULL
impl Ptr<u64> for XV2ULL {
  /// ptr
  #[inline]
  fn ptr(&self) -> *const u64 { self as *const XV2ULL as *const u64 }
  /// ptr_mut
  #[inline]
  fn ptr_mut(&mut self) -> *mut u64 { self as *mut XV2ULL as *mut u64 }
}

/// trait Dump for XV2ULL
impl Dump for XV2ULL {
  /// dump
  fn dump(&self) -> String {
    let mut p = DispMatParam{m: self.voidp(), w: SZU64, rows: 1, cols: 2,
      fmt: "%016llx\0".as_ptr(), di: 16, df: 0, r: 0, c: 0};
    let f = cb_xll;
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Disp for XV2ULL
impl Disp for XV2ULL {
  /// disp
  fn disp(&self, di: usize, df: usize) -> String {
    let fmt = format!("%{}llu\0", di).as_str().as_ptr();
    let mut p = DispMatParam{m: self.voidp(), w: SZU64, rows: 1, cols: 2,
      fmt, di, df, r: 0, c: 0};
    let f = cb_u64; // (by di) same as cb_ull (by fmt)
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Display for XV2ULL
impl std::fmt::Display for XV2ULL {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(21, 0))
  }
}

/// trait Debug for XV2ULL
impl std::fmt::Debug for XV2ULL {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(21, 0))
  }
}

/// XV2LL
#[derive(Clone)]
pub struct XV2LL (pub [ffi::c_longlong; 2]);

/// trait Ptr for XV2LL
impl Ptr<i64> for XV2LL {
  /// ptr
  #[inline]
  fn ptr(&self) -> *const i64 { self as *const XV2LL as *const i64 }
  /// ptr_mut
  #[inline]
  fn ptr_mut(&mut self) -> *mut i64 { self as *mut XV2LL as *mut i64 }
}

/// trait Dump for XV2LL
impl Dump for XV2LL {
  /// dump
  fn dump(&self) -> String {
    let mut p = DispMatParam{m: self.voidp(), w: SZI64, rows: 1, cols: 2,
      fmt: "%016llx\0".as_ptr(), di: 16, df: 0, r: 0, c: 0};
    let f = cb_xll;
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Disp for XV2LL
impl Disp for XV2LL {
  /// disp
  fn disp(&self, di: usize, df: usize) -> String {
    let fmt = format!("%{}lld\0", di).as_str().as_ptr();
    let mut p = DispMatParam{m: self.voidp(), w: SZI64, rows: 1, cols: 2,
      fmt, di, df, r: 0, c: 0};
    let f = cb_i64; // (by di) same as cb_ll (by fmt)
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Display for XV2LL
impl std::fmt::Display for XV2LL {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(21, 0))
  }
}

/// trait Debug for XV2LL
impl std::fmt::Debug for XV2LL {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(21, 0))
  }
}

/// XV2U
#[derive(Clone)]
pub struct XV2U (pub [ffi::c_uint; 2]);

/// trait Ptr for XV2U
impl Ptr<u32> for XV2U {
  /// ptr
  #[inline]
  fn ptr(&self) -> *const u32 { self as *const XV2U as *const u32 }
  /// ptr_mut
  #[inline]
  fn ptr_mut(&mut self) -> *mut u32 { self as *mut XV2U as *mut u32 }
}

/// trait Dump for XV2U
impl Dump for XV2U {
  /// dump
  fn dump(&self) -> String {
    let mut p = DispMatParam{m: self.voidp(), w: SZU32, rows: 1, cols: 2,
      fmt: "%08x\0".as_ptr(), di: 8, df: 0, r: 0, c: 0};
    let f = cb_x;
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Disp for XV2U
impl Disp for XV2U {
  /// disp
  fn disp(&self, di: usize, df: usize) -> String {
    let fmt = format!("%{}lu\0", di).as_str().as_ptr();
    let mut p = DispMatParam{m: self.voidp(), w: SZU32, rows: 1, cols: 2,
      fmt, di, df, r: 0, c: 0};
    let f = cb_u32; // (by di) same as cb_u (by fmt)
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Display for XV2U
impl std::fmt::Display for XV2U {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(17, 0))
  }
}

/// trait Debug for XV2U
impl std::fmt::Debug for XV2U {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(17, 0))
  }
}

/// XV2I
#[derive(Clone)]
pub struct XV2I (pub [ffi::c_int; 2]);

/// trait Ptr for XV2I
impl Ptr<i32> for XV2I {
  /// ptr
  #[inline]
  fn ptr(&self) -> *const i32 { self as *const XV2I as *const i32 }
  /// ptr_mut
  #[inline]
  fn ptr_mut(&mut self) -> *mut i32 { self as *mut XV2I as *mut i32 }
}

/// trait Dump for XV2I
impl Dump for XV2I {
  /// dump
  fn dump(&self) -> String {
    let mut p = DispMatParam{m: self.voidp(), w: SZI32, rows: 1, cols: 2,
      fmt: "%08x\0".as_ptr(), di: 8, df: 0, r: 0, c: 0};
    let f = cb_x;
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Disp for XV2I
impl Disp for XV2I {
  /// disp
  fn disp(&self, di: usize, df: usize) -> String {
    let fmt = format!("%{}ld\0", di).as_str().as_ptr();
    let mut p = DispMatParam{m: self.voidp(), w: SZI32, rows: 1, cols: 2,
      fmt, di, df, r: 0, c: 0};
    let f = cb_i32; // (by di) same as cb_i (by fmt)
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Display for XV2I
impl std::fmt::Display for XV2I {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(17, 0))
  }
}

/// trait Debug for XV2I
impl std::fmt::Debug for XV2I {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(17, 0))
  }
}

/// XV2F32
#[derive(Clone)]
pub struct XV2F32 (pub [ffi::c_float; 2]);

/// trait Ptr for XV2F32
impl Ptr<f32> for XV2F32 {
  /// ptr
  #[inline]
  fn ptr(&self) -> *const f32 { self as *const XV2F32 as *const f32 }
  /// ptr_mut
  #[inline]
  fn ptr_mut(&mut self) -> *mut f32 { self as *mut XV2F32 as *mut f32 }
}

/// trait Dump for XV2F32
impl Dump for XV2F32 {
  /// dump
  fn dump(&self) -> String {
    let mut p = DispMatParam{m: self.voidp(), w: SZF32, rows: 1, cols: 2,
      fmt: "%08x\0".as_ptr(), di: 8, df: 0, r: 0, c: 0};
    let f = cb_x;
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Disp for XV2F32
impl Disp for XV2F32 {
  /// disp
  fn disp(&self, di: usize, df: usize) -> String {
    let fmt = format!("%{}.{}f\0", di, df).as_str().as_ptr();
    let mut p = DispMatParam{m: self.voidp(), w: SZF32, rows: 1, cols: 2,
      fmt, di, df, r: 0, c: 0};
    let f = cb_f32; // (by di df) same as cb_f (by fmt)
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Display for XV2F32
impl std::fmt::Display for XV2F32 {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(17, 7))
  }
}

/// trait Debug for XV2F32
impl std::fmt::Debug for XV2F32 {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(17, 7))
  }
}

/// XV3F32
#[derive(Clone)]
pub struct XV3F32 (pub [ffi::c_float; 3]);

/// XV3F32
impl XV3F32 {
  /// o (create new instance)
  #[inline]
  pub fn o() -> Self {
    XV3F32([0.0f32; 3])
  }

  /// prec_diff (create new instance)
  pub fn prec_diff(&self, b: &Self, p: f32) -> Result<Self, Box<dyn Error>> {
    Ok(XV3F32(
/*
      toarray!(self.0.iter().enumerate().map(|(i, &f)|
        prec_diff(f, b.0[i], p)))
*/
      toarray!(self.0.iter().zip(b.0.iter()).map(|(&f, &g)|
        prec_diff(f, g, p)))
    ))
  }

  /// prec_eq
  #[inline]
  pub fn prec_eq(&self, b: &Self, p: f32) ->
    Result<bool, Box<dyn Error>> {
    Ok(self.prec_diff(b, p)?.0 == XV3F32::o().0)
  }

  /// prec_eq_array
  #[inline]
  pub fn prec_eq_array(&self, b: &[f32; 3], p: f32) ->
    Result<bool, Box<dyn Error>> {
    self.prec_eq(&XV3F32(*b), p)
  }

  /// dot (create new instance)
  pub fn dot(&self, b: &Self) -> f32 {
    self.0.iter().zip(b.0.iter()).map(|(&f, &g)| f * g).sum()
  }

  /// cross (create new instance)
  pub fn cross(&self, b: &Self) -> Self {
    let [x0, y0, z0] = self.0;
    let [x1, y1, z1] = b.0;
    XV3F32([y0 * z1 - z0 * y1, z0 * x1 - x0 * z1, x0 * y1 - y0 * x1])
  }

  /// norm (create new instance)
  #[inline]
  pub fn norm(&self, b: &Self) -> Result<Self, Box<dyn Error>> {
    self.cross(b).normalize()
  }

  /// normalize (create new instance)
  pub fn normalize(&self) -> Result<Self, Box<dyn Error>> {
    let a = self.0;
    let r = f32::sqrt(a.into_iter().map(|f| f * f).sum());
    Ok(XV3F32(toarray!(a.into_iter().map(|f| f / r))))
  }

  /// set_normalize
  #[inline]
  pub fn set_normalize(&mut self) -> Result<&mut Self, Box<dyn Error>> {
    self.0 = self.normalize()?.0;
    Ok(self)
  }
}

/// trait TryFrom for XV3F32
impl TryFrom<XV4F32> for XV3F32 {
  type Error = Box<dyn Error>;

  /// try_from (create new instance)
  fn try_from(v: XV4F32) -> Result<Self, Self::Error> {
    let v = v.0;
    Ok(XV3F32([v[0], v[1], v[2]])) // no care v[3] (assume always 1)
  }
}

/// trait Ptr for XV3F32
impl Ptr<f32> for XV3F32 {
  /// ptr
  #[inline]
  fn ptr(&self) -> *const f32 { self as *const XV3F32 as *const f32 }
  /// ptr_mut
  #[inline]
  fn ptr_mut(&mut self) -> *mut f32 { self as *mut XV3F32 as *mut f32 }
}

/// trait Dump for XV3F32
impl Dump for XV3F32 {
  /// dump
  fn dump(&self) -> String {
    let mut p = DispMatParam{m: self.voidp(), w: SZF32, rows: 1, cols: 3,
      fmt: "%08x\0".as_ptr(), di: 8, df: 0, r: 0, c: 0};
    let f = cb_x;
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Disp for XV3F32
impl Disp for XV3F32 {
  /// disp
  fn disp(&self, di: usize, df: usize) -> String {
    let fmt = format!("%{}.{}f\0", di, df).as_str().as_ptr();
    let mut p = DispMatParam{m: self.voidp(), w: SZF32, rows: 1, cols: 3,
      fmt, di, df, r: 0, c: 0};
    let f = cb_f32; // (by di df) same as cb_f (by fmt)
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Display for XV3F32
impl std::fmt::Display for XV3F32 {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(17, 7))
  }
}

/// trait Debug for XV3F32
impl std::fmt::Debug for XV3F32 {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(17, 7))
  }
}

/// XV4F32
#[derive(Clone)]
pub struct XV4F32 (pub [ffi::c_float; 4]);

/// XV4F32
impl XV4F32 {
  /// o (create new instance)
  #[inline]
  pub fn o() -> Self {
    XV4F32([0.0f32; 4])
  }

  /// i as quaternion (create new instance)
  #[inline]
  pub fn i() -> Self {
    XV4F32([0.0f32, 0.0f32, 0.0f32, 1.0f32])
  }

  /// prec_diff (create new instance)
  pub fn prec_diff(&self, b: &Self, p: f32) -> Result<Self, Box<dyn Error>> {
    Ok(XV4F32(
/*
      toarray!(self.0.iter().enumerate().map(|(i, &f)|
        prec_diff(f, b.0[i], p)))
*/
      toarray!(self.0.iter().zip(b.0.iter()).map(|(&f, &g)|
        prec_diff(f, g, p)))
    ))
  }

  /// prec_eq
  #[inline]
  pub fn prec_eq(&self, b: &Self, p: f32) ->
    Result<bool, Box<dyn Error>> {
    Ok(self.prec_diff(b, p)?.0 == XV4F32::o().0)
  }

  /// prec_eq_array
  #[inline]
  pub fn prec_eq_array(&self, b: &[f32; 4], p: f32) ->
    Result<bool, Box<dyn Error>> {
    self.prec_eq(&XV4F32(*b), p)
  }

  /// dot (create new instance)
  pub fn dot(self, b: &Self) -> f32 {
    self.0.iter().zip(b.0.iter()).map(|(&f, &g)| f * g).sum()
  }

  /// cross3 (create new instance)
  pub fn cross3(self, b: &Self) -> Self {
    let [x0, y0, z0, _w0] = self.0;
    let [x1, y1, z1, _w1] = b.0;
    let w = 1.0f32; // no care w0 w1
    XV4F32([y0 * z1 - z0 * y1, z0 * x1 - x0 * z1, x0 * y1 - y0 * x1, w])
  }

  /// norm3 (create new instance)
  #[inline]
  pub fn norm3(self, b: &Self) -> Result<Self, Box<dyn Error>> {
    self.cross3(b).normalize3()
  }

  /// normalize3 as quaternion (create new instance)
  #[inline]
  pub fn normalize3(self) -> Result<Self, Box<dyn Error>> {
    XV4F32::try_from(XV3F32::try_from(self)?.normalize()?)
  }

  /// normalize as XV4F32 (create new instance)
  pub fn normalize(&self) -> Result<Self, Box<dyn Error>> {
    let mut q = Self::o();
unsafe {
    if isnull!(D3DXQuaternionNormalize(
      q.ptr_mut() as *mut D3DXQUATERNION,
      self.ptr() as *const D3DXQUATERNION
    )) { Err("normalize".into()) }
    else { Ok(q) }
}
  }

  /// set_normalize as XV4F32
  pub fn set_normalize(&mut self) -> Result<&mut Self, Box<dyn Error>> {
unsafe {
    if isnull!(D3DXQuaternionNormalize(
      self.ptr_mut() as *mut D3DXQUATERNION,
      self.ptr() as *const D3DXQUATERNION
    )) { Err("set_normalize".into()) }
    else { Ok(self) }
}
  }

  /// inverse (create new instance)
  pub fn inverse(&self) -> Result<Self, Box<dyn Error>> {
    let mut q = Self::o();
unsafe {
    if isnull!(D3DXQuaternionInverse(
      q.ptr_mut() as *mut D3DXQUATERNION,
      self.ptr() as *const D3DXQUATERNION
    )) { Err("inverse".into()) }
    else { Ok(q) }
}
  }

  /// conjugate (create new instance) [i, j, k, r] instead of [r, i, j, k]
  #[inline]
  pub fn conjugate(&self) -> Result<Self, Box<dyn Error>> {
    let q = self.0;
    Ok(XV4F32([-q[0], -q[1], -q[2], q[3]]))
//    Ok(XV4F32([q[0], -q[1], -q[2], -q[3]]))
  }

  /// quaternion (create new instance) [i, j, k, r] instead of [r, i, j, k]
  pub fn quaternion(axis: &XV3F32, th: f32) -> Result<Self, Box<dyn Error>> {
    let a = axis.normalize()?.0;
    let r = th / 2.0f32;
    let c = f32::cos(r);
    let s = f32::sin(r);
    Ok(XV4F32([a[0] * s, a[1] * s, a[2] * s, c]))
//    Ok(XV4F32([c, a[0] * s, a[1] * s, a[2] * s]))
  }

  /// quaternion_m (create new instance) [i, j, k, r] instead of [r, i, j, k]
  pub fn quaternion_m(m: &XMat44F32, prec: f32) ->
    Result<Self, Box<dyn Error>> {
    let q = m.transpose()?.0; // transposed from Qt for DirectX (mul P Qt)
    let (xs, ys, zs, c); // needless mut when all will be assigned once
    let cc = (q[0][0] + q[1][1] + q[2][2] + q[3][3]) / 4.0f32;
    if prec_eq(cc, 0.0f32, prec) {
      c = 0.0f32;
      let xsxs = - (q[1][1] + q[2][2]) / 2.0f32;
      if prec_eq(xsxs, 0.0f32, prec) {
        xs = 0.0f32;
        let ysys = (q[3][3] - q[2][2]) / 2.0f32;
        if prec_eq(ysys, 0.0f32, prec) {
          ys = 0.0f32;
          zs = 1.0f32;
        }else if ysys > 0.0f32 {
          ys = f32::sqrt(ysys); // positive only
          zs = q[1][2] / (ys * 2.0f32);
        }else{ // ysys < 0.0f32
          return Err("not a rotation matrix".into());
        }
      }else if xsxs > 0.0f32 {
        xs = f32::sqrt(xsxs); // positive only
        let xs2 = xs * 2.0f32;
        ys = q[0][1] / xs2;
        zs = q[0][2] / xs2;
      }else{ // xsxs < 0.0f32
        return Err("not a rotation matrix".into());
      }
    }else if cc > 0.0f32 {
      c = f32::sqrt(cc); // positive only
      let c4 = c * 4.0f32;
      xs = (q[2][1] - q[1][2]) / c4;
      ys = (q[0][2] - q[2][0]) / c4;
      zs = (q[1][0] - q[0][1]) / c4;
    }else{ // cc < 0.0f32
      return Err("not a rotation matrix".into());
    }
    Ok(XV4F32([xs, ys, zs, c]))
  }

  /// rotation_axis (quaternion rotation axis) (create new instance)
  pub fn rotation_axis(axis: &XV3F32, th: f32) ->
    Result<Self, Box<dyn Error>> {
    let mut q = Self::o();
unsafe {
    if isnull!(D3DXQuaternionRotationAxis(
      q.ptr_mut() as *mut D3DXQUATERNION,
      axis.ptr() as *const D3DXVECTOR3,
      th
    )) { Err("quaternion rotation axis".into()) }
    else { Ok(q) }
}
  }

  /// set_rotation_axis (quaternion rotation axis)
  pub fn set_rotation_axis(&mut self, axis: &XV3F32, th: f32) ->
    Result<&mut Self, Box<dyn Error>> {
unsafe {
    if isnull!(D3DXQuaternionRotationAxis(
      self.ptr_mut() as *mut D3DXQUATERNION,
      axis.ptr() as *const D3DXVECTOR3,
      th
    )) { Err("quaternion set rotation axis".into()) }
    else { Ok(self) }
}
  }

  /// mul (create new instance) [i, j, k, r] instead of [r, i, j, k]
  /// mul quaternion and quaternion (***NOT same as dot***)
  /// a.mul(b) means (a * b) for GL (b * a) for DirectX
  #[inline]
  pub fn mul(&self, b: &Self) ->
    Result<Self, Box<dyn Error>> {
    XMat44F32::quaternion_r(self)?.mulmv(b) // (mul Qr(a) v(b)) for DirectX
//    XMat44F32::quaternion_l(b)?.mulmv(self) // (mul Ql(b) v(a)) for DirectX
//    XMat44F32::quaternion_l(self)?.mulmv(b) // (mul Ql(a) v(b)) for GL differ
  }

  /// q_mul (quaternion multiply) (create new instance) (***NOT same as dot***)
  pub fn q_mul(&self, b: &Self) ->
    Result<Self, Box<dyn Error>> {
    let mut q = Self::o();
unsafe {
    if isnull!(D3DXQuaternionMultiply(
      q.ptr_mut() as *mut D3DXQUATERNION,
      self.ptr() as *const D3DXQUATERNION,
      b.ptr() as *const D3DXQUATERNION
    )) { Err("quaternion multiply".into()) }
    else { Ok(q) }
}
  }

  /// set_q_mul (quaternion multiply)
  pub fn set_q_mul(&mut self, b: &Self) ->
    Result<&mut Self, Box<dyn Error>> {
unsafe {
    if isnull!(D3DXQuaternionMultiply(
      self.ptr_mut() as *mut D3DXQUATERNION,
      self.ptr() as *const D3DXQUATERNION,
      b.ptr() as *const D3DXQUATERNION
    )) { Err("set_q_mul".into()) }
    else { Ok(self) }
}
  }
}

/// trait TryFrom for XV4F32
impl TryFrom<XMat44F32> for XV4F32 {
  type Error = Box<dyn Error>;

  /// try_from rotation matrix into quaternion (create new instance)
  fn try_from(m: XMat44F32) -> Result<Self, Self::Error> {
    let mut q = Self::o();
unsafe {
    if isnull!(D3DXQuaternionRotationMatrix(
      q.ptr_mut() as *mut D3DXQUATERNION,
      m.ptr() as *const D3DXMATRIX
    )) { Err("quaternion rotation matrix".into()) }
    else { Ok(q) }
}
  }
}

/// trait TryFrom for XV4F32
impl TryFrom<XV3F32> for XV4F32 {
  type Error = Box<dyn Error>;

  /// try_from (create new instance)
  fn try_from(v: XV3F32) -> Result<Self, Self::Error> {
    let v = v.0;
    Ok(XV4F32([v[0], v[1], v[2], 1.0f32])) // force set v[3] (assume always 1)
  }
}

/// trait Ptr for XV4F32
impl Ptr<f32> for XV4F32 {
  /// ptr
  #[inline]
  fn ptr(&self) -> *const f32 { self as *const XV4F32 as *const f32 }
  /// ptr_mut
  #[inline]
  fn ptr_mut(&mut self) -> *mut f32 { self as *mut XV4F32 as *mut f32 }
}

/// trait Dump for XV4F32
impl Dump for XV4F32 {
  /// dump
  fn dump(&self) -> String {
    let mut p = DispMatParam{m: self.voidp(), w: SZF32, rows: 1, cols: 4,
      fmt: "%08x\0".as_ptr(), di: 8, df: 0, r: 0, c: 0};
    let f = cb_x;
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Disp for XV4F32
impl Disp for XV4F32 {
  /// disp
  fn disp(&self, di: usize, df: usize) -> String {
    let fmt = format!("%{}.{}f\0", di, df).as_str().as_ptr();
    let mut p = DispMatParam{m: self.voidp(), w: SZF32, rows: 1, cols: 4,
      fmt, di, df, r: 0, c: 0};
    let f = cb_f32; // (by di df) same as cb_f (by fmt)
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Display for XV4F32
impl std::fmt::Display for XV4F32 {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(17, 7))
  }
}

/// trait Debug for XV4F32
impl std::fmt::Debug for XV4F32 {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(17, 7))
  }
}
