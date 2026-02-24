//! xmat.rs

use crate::core::*;
// use crate::w32a::*;
use std::{ffi};
use std::{result::Result, error::Error};
use windows::{
  core::*
};

/// prec_eq_c_array macro ($a: XMat44F32, $b: float \[\]\[4\], $p: f32)
#[macro_export]
macro_rules! prec_eq_c_array {
  ($a:expr, $b:expr, $p:expr) => {{
    let m = unsafe { &$b } as *const [[f32; 4]; 0];
    $a.prec_eq(&XMat44F32::from_float_m04(m), $p)
  }};
}
pub use prec_eq_c_array;

/// prec_eq_c_array_transposed macro ($a: XMat44F32, $b: float \[\]\[4\], $p: f32)
#[macro_export]
macro_rules! prec_eq_c_array_transposed {
  ($a:expr, $b:expr, $p:expr) => {
    prec_eq_c_array!($a.transpose()?, $b, $p)
  };
}
pub use prec_eq_c_array_transposed;

/// XMat44F32
#[derive(Clone)]
pub struct XMat44F32 (pub [[ffi::c_float; 4]; 4]); // [XV4F32; 4]

/// XMat44F32
impl XMat44F32 {
  /// o (create new instance)
  #[inline]
  pub fn o() -> Self {
    XMat44F32([[0.0f32; 4]; 4])
  }

  /// set_from (use clone to create new instance)
  #[inline]
  pub fn set_from(&mut self, b: &Self) -> &mut Self {
    self.0 = b.0;
    self
  }

  /// from_float_p (create new instance)
  #[inline]
  pub fn from_float_p(m: *const f32) -> Self {
    XMat44F32::from_float_m4(m as *const [f32; 4])
  }

  /// from_float_m0 (create new instance)
  #[inline]
  pub fn from_float_m0(m: *const [f32; 0]) -> Self {
    XMat44F32::from_float_m4(m as *const [f32; 4])
  }

  /// from_float_m4 (create new instance)
  pub fn from_float_m4(m: *const [f32; 4]) -> Self {
    XMat44F32(unsafe { std::slice::from_raw_parts(m, 4) }
      .try_into().expect("float m44"))
  }

  /// from_float_m04 (create new instance)
  #[inline]
  pub fn from_float_m04(m: *const [[f32; 4]; 0]) -> Self {
    XMat44F32::from_float_m4(m as *const [f32; 4])
  }

  /// from_float_m44 (create new instance)
  #[inline]
  pub fn from_float_m44(m: *const [[f32; 4]; 4]) -> Self {
    XMat44F32::from_float_m4(m as *const [f32; 4])
  }

  /// prec_diff (create new instance)
  pub fn prec_diff(&self, b: &Self, p: f32) -> Result<Self, Box<dyn Error>> {
    Ok(XMat44F32(
/*
      toarray!(self.0.iter().enumerate().map(|(j, &v)|
        toarray!(v.iter().enumerate().map(|(i, &f)|
          prec_diff(f, b.0[j][i], p)))))
*/
      toarray!(self.0.iter().zip(b.0.iter()).map(|(&vf, &vg)|
        toarray!(vf.iter().zip(vg.iter()).map(|(&f, &g)|
          prec_diff(f, g, p)))))
    ))
  }

  /// prec_eq
  #[inline]
  pub fn prec_eq(&self, b: &Self, p: f32) ->
    Result<bool, Box<dyn Error>> {
    Ok(self.prec_diff(b, p)?.0 == XMat44F32::o().0)
  }

  /// prec_eq_transposed
  #[inline]
  pub fn prec_eq_transposed(&self, b: &Self, p: f32) ->
    Result<bool, Box<dyn Error>> {
    self.transpose()?.prec_eq(b, p)
  }

  /// prec_eq_array
  #[inline]
  pub fn prec_eq_array(&self, b: &[[f32; 4]; 4], p: f32) ->
    Result<bool, Box<dyn Error>> {
    self.prec_eq(&XMat44F32(*b), p)
  }

  /// prec_eq_array_transposed
  #[inline]
  pub fn prec_eq_array_transposed(&self, b: &[[f32; 4]; 4], p: f32) ->
    Result<bool, Box<dyn Error>> {
    self.transpose()?.prec_eq(&XMat44F32(*b), p)
  }

  /// i (create new instance)
  pub fn i() -> Result<Self, Box<dyn Error>> {
    let mut m = Self::o();
unsafe {
    if isnull!(D3DXMatrixIdentity(
      m.ptr_mut() as *mut D3DXMATRIX
    )) { Err("identity".into()) }
    else { Ok(m) }
}
  }

  /// set_i
  pub fn set_i(&mut self) -> Result<&mut Self, Box<dyn Error>> {
unsafe {
    if isnull!(D3DXMatrixIdentity(
      self.ptr_mut() as *mut D3DXMATRIX
    )) { Err("set_identity".into()) }
    else { Ok(self) }
}
  }

  /// transpose (create new instance)
  pub fn transpose(&self) -> Result<Self, Box<dyn Error>> {
    let mut m = Self::o();
unsafe {
    if isnull!(D3DXMatrixTranspose(
      m.ptr_mut() as *mut D3DXMATRIX,
      self.ptr() as *const D3DXMATRIX
    )) { Err("transpose".into()) }
    else { Ok(m) }
}
  }

  /// set_transpose
  pub fn set_transpose(&mut self) -> Result<&mut Self, Box<dyn Error>> {
unsafe {
    if isnull!(D3DXMatrixTranspose(
      self.ptr_mut() as *mut D3DXMATRIX,
      self.ptr() as *const D3DXMATRIX
    )) { Err("set_transpose".into()) }
    else { Ok(self) }
}
  }

  /// det
  #[inline]
  pub fn det(&self, prec: f32) -> Result<f32, Box<dyn Error>> {
    let d = unsafe { D3DXMatrixDeterminant(self.ptr() as *const D3DXMATRIX) };
    if prec_eq(d, 0.0f32, prec) { Ok(0.0f32) } else { Ok(d) }
  }

  /// inverse (create new instance)
  pub fn inverse(&self, det: Option<&mut f32>) ->
    Result<Self, Box<dyn Error>> {
    let mut m = Self::o();
unsafe {
    if isnull!(D3DXMatrixInverse(
      m.ptr_mut() as *mut D3DXMATRIX,
      match det { None => 0 as *mut f32, Some(det) => det as *mut f32 },
      self.ptr() as *const D3DXMATRIX
    )) { Err("inverse".into()) }
    else { Ok(m) }
}
  }

  /// set_inverse
  pub fn set_inverse(&mut self, det: Option<&mut f32>) ->
    Result<&mut Self, Box<dyn Error>> {
unsafe {
    if isnull!(D3DXMatrixInverse(
      self.ptr_mut() as *mut D3DXMATRIX,
      match det { None => 0 as *mut f32, Some(det) => det as *mut f32 },
      self.ptr() as *const D3DXMATRIX
    )) { Err("set_inverse".into()) }
    else { Ok(self) }
}
  }

  /// mul (create new instance)
  pub fn mul(&self, b: &Self) -> Result<Self, Box<dyn Error>> {
    let mut m = Self::o();
unsafe {
    if isnull!(D3DXMatrixMultiply(
      m.ptr_mut() as *mut D3DXMATRIX,
      self.ptr() as *const D3DXMATRIX,
      b.ptr() as *const D3DXMATRIX
    )) { Err("mul".into()) }
    else { Ok(m) }
}
  }

  /// set_mul
  pub fn set_mul(&mut self, b: &Self) -> Result<&mut Self, Box<dyn Error>> {
unsafe {
    if isnull!(D3DXMatrixMultiply(
      self.ptr_mut() as *mut D3DXMATRIX,
      self.ptr() as *const D3DXMATRIX,
      b.ptr() as *const D3DXMATRIX
    )) { Err("set_mul".into()) }
    else { Ok(self) }
}
  }

  /// translation (create new instance)
  pub fn translation(x: f32, y: f32, z: f32) ->
    Result<Self, Box<dyn Error>> {
    let mut m = Self::o();
unsafe {
    if isnull!(D3DXMatrixTranslation(
      m.ptr_mut() as *mut D3DXMATRIX, x, y, z
    )) { Err("translation".into()) }
    else { Ok(m) }
}
  }

  /// set_translation
  pub fn set_translation(&mut self, x: f32, y: f32, z: f32) ->
    Result<&mut Self, Box<dyn Error>> {
unsafe {
    if isnull!(D3DXMatrixTranslation(
      self.ptr_mut() as *mut D3DXMATRIX, x, y, z
    )) { Err("set_translation".into()) }
    else { Ok(self) }
}
  }

  /// scaling (create new instance)
  pub fn scaling(x: f32, y: f32, z: f32) ->
    Result<Self, Box<dyn Error>> {
    let mut m = Self::o();
unsafe {
    if isnull!(D3DXMatrixScaling(
      m.ptr_mut() as *mut D3DXMATRIX, x, y, z
    )) { Err("scaling".into()) }
    else { Ok(m) }
}
  }

  /// set_scaling
  pub fn set_scaling(&mut self, x: f32, y: f32, z: f32) ->
    Result<&mut Self, Box<dyn Error>> {
unsafe {
    if isnull!(D3DXMatrixScaling(
      self.ptr_mut() as *mut D3DXMATRIX, x, y, z
    )) { Err("set_scaling".into()) }
    else { Ok(self) }
}
  }

  /// rotation_axis (matrix rotation axis) (create new instance)
  pub fn rotation_axis(v: &XV3F32, r: f32) ->
    Result<Self, Box<dyn Error>> {
    let mut m = Self::o();
unsafe {
    if isnull!(D3DXMatrixRotationAxis(
      m.ptr_mut() as *mut D3DXMATRIX,
      v.ptr() as *const D3DXVECTOR3,
      r
    )) { Err("matrix rotation axis".into()) }
    else { Ok(m) }
}
  }

  /// set_rotation_axis (matrix rotation axis)
  pub fn set_rotation_axis(&mut self, v: &XV3F32, r: f32) ->
    Result<&mut Self, Box<dyn Error>> {
unsafe {
    if isnull!(D3DXMatrixRotationAxis(
      self.ptr_mut() as *mut D3DXMATRIX,
      v.ptr() as *const D3DXVECTOR3,
      r
    )) { Err("matrix set rotation axis".into()) }
    else { Ok(self) }
}
  }

  /// look_at_lh (create new instance)
  pub fn look_at_lh(eye: &XV3F32, lookat: &XV3F32, up: &XV3F32) ->
    Result<Self, Box<dyn Error>> {
    let mut m = Self::o();
unsafe {
    if isnull!(D3DXMatrixLookAtLH(
      m.ptr_mut() as *mut D3DXMATRIX,
      eye.ptr() as *const D3DXVECTOR3,
      lookat.ptr() as *const D3DXVECTOR3,
      up.ptr() as *const D3DXVECTOR3
    )) { Err("?".into()) }
    else { Ok(m) }
}
  }

  /// perspective_fov_lh (create new instance)
  pub fn perspective_fov_lh(r: f32, ratio: f32, z0: f32, z1: f32) ->
    Result<Self, Box<dyn Error>> {
    let mut m = Self::o();
unsafe {
    if isnull!(D3DXMatrixPerspectiveFovLH(
      m.ptr_mut() as *mut D3DXMATRIX,
      r, ratio, z0, z1
    )) { Err("?".into()) }
    else { Ok(m) }
}
  }

  /// mulmv (create new instance) [i, j, k, r] instead of [r, i, j, k]
  /// row major
  pub fn mulmv(&self, v: &XV4F32) -> Result<XV4F32, Box<dyn Error>> {
    Ok(XV4F32(toarray!(self.0.iter().map(|&r| XV4F32(r).dot(v))))) // (mul M v)
  }

  /// mulvm (create new instance) [i, j, k, r] instead of [r, i, j, k]
  /// colmun major
  #[inline]
  pub fn mulvm(&self, v: &XV4F32) -> Result<XV4F32, Box<dyn Error>> {
    self.transpose()?.mulmv(v) // (mul v M)
  }

  /// quaternion (create new instance) [i, j, k, r] instead of [r, i, j, k]
  pub fn quaternion(axis: &XV3F32, th: f32) -> Result<Self, Box<dyn Error>> {
    let q = XV4F32::quaternion(axis, th)?;
    let q_l = XMat44F32::quaternion_l(&q)?;
    let q_r = XMat44F32::quaternion_r(&q.conjugate()?)?;
    let m = q_r.mul(&q_l);
//    m // for GL (mul (Qr Ql) P) to be transposed column major
//    q_r.transpose()?.mul(&q_l.transpose()?) // for DirectX (mul P (Qrt Qlt))
    m?.transpose() // fast for DirectX (mul P (Qr Ql)t)
  }

  /// set_quaternion
  #[inline]
  pub fn set_quaternion(&mut self, axis: &XV3F32, th: f32) ->
    Result<&mut Self, Box<dyn Error>> {
    self.0 = XMat44F32::quaternion(axis, th)?.0;
    Ok(self)
  }

  /// quaternion_l (create new instance) [i, j, k, r] instead of [r, i, j, k]
  pub fn quaternion_l(q: &XV4F32) -> Result<Self, Box<dyn Error>> {
    let q = q.0;
    Ok(XMat44F32([
      [ q[3], -q[2],  q[1], q[0]], // [q[0], -q[1], -q[2], -q[3]],
      [ q[2],  q[3], -q[0], q[1]], // [q[1],  q[0], -q[3],  q[2]],
      [-q[1],  q[0],  q[3], q[2]], // [q[2],  q[3],  q[0], -q[1]],
      [-q[0], -q[1], -q[2], q[3]]  // [q[3], -q[2],  q[1],  q[0]]
    ])) // for GL (mul Qr Ql P)
  }

  /// set_quaternion_l
  #[inline]
  pub fn set_quaternion_l(&mut self, q: &XV4F32) ->
    Result<&mut Self, Box<dyn Error>> {
    self.0 = XMat44F32::quaternion_l(q)?.0;
    Ok(self)
  }

  /// quaternion_r (create new instance) [i, j, k, r] instead of [r, i, j, k]
  pub fn quaternion_r(q: &XV4F32) -> Result<Self, Box<dyn Error>> {
    let q = q.0;
    Ok(XMat44F32([
      [ q[3],  q[2], -q[1], q[0]], // [q[0], -q[1], -q[2], -q[3]],
      [-q[2],  q[3],  q[0], q[1]], // [q[1],  q[0],  q[3], -q[2]],
      [ q[1], -q[0],  q[3], q[2]], // [q[2], -q[3],  q[0],  q[1]],
      [-q[0], -q[1], -q[2], q[3]]  // [q[3],  q[2], -q[1],  q[0]]
    ])) // for GL (mul Qr Ql P)
  }

  /// set_quaternion_r
  #[inline]
  pub fn set_quaternion_r(&mut self, q: &XV4F32) ->
    Result<&mut Self, Box<dyn Error>> {
    self.0 = XMat44F32::quaternion_r(q)?.0;
    Ok(self)
  }

  /// rot_cg (create new instance)
  pub fn rot_cg(axis: &XV3F32, a: f32, cg: &XV3F32) ->
    Result<Self, Box<dyn Error>> {
unsafe {
    let mut m = Self::o();
    if failed!(HRESULT(rotCG(
      m.ptr_mut() as *mut D3DXMATRIX,
      axis.ptr() as *const D3DXVECTOR3,
      a,
      cg.ptr() as *const D3DXVECTOR3
    ))) { Err("rotCG".into()) }
    else { Ok(m) }
}
  }
}

/// trait TryFrom for XMat44F32
impl TryFrom<XV4F32> for XMat44F32 {
  type Error = Box<dyn Error>;

  /// try_from rotation quaternion into matrix (create new instance)
  fn try_from(q: XV4F32) -> Result<Self, Self::Error> {
    let mut m = Self::o();
unsafe {
    if isnull!(D3DXMatrixRotationQuaternion(
      m.ptr_mut() as *mut D3DXMATRIX,
      q.ptr() as *const D3DXQUATERNION
    )) { Err("matrix rotation quaternion".into()) }
    else { Ok(m) }
}
  }
}

/// trait Ptr for XMat44F32
impl Ptr<f32> for XMat44F32 {
  /// ptr
  #[inline]
  fn ptr(&self) -> *const f32 { self as *const XMat44F32 as *const f32 }
  /// ptr_mut
  #[inline]
  fn ptr_mut(&mut self) -> *mut f32 { self as *mut XMat44F32 as *mut f32 }
}

/// trait Dump for XMat44F32
impl Dump for XMat44F32 {
  /// dump
  fn dump(&self) -> String {
    let mut p = DispMatParam{m: self.voidp(), w: SZF32, rows: 4, cols: 4,
      fmt: "%08x\0".as_ptr(), di: 8, df: 0, r: 0, c: 0};
    let f = cb_x;
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Disp for XMat44F32
impl Disp for XMat44F32 {
  /// disp
  fn disp(&self, di: usize, df: usize) -> String {
    let fmt = format!("%{}.{}f\0", di, df).as_str().as_ptr();
    let mut p = DispMatParam{m: self.voidp(), w: SZF32, rows: 4, cols: 4,
      fmt, di, df, r: 0, c: 0};
    let f = cb_f32; // (by di df) same as cb_f (by fmt)
    let mut s = [0u8; SZBUF];
unsafe {
    let l = disp_mat(&mut s as *mut u8, SZBUF, &mut p, Some(f));
    String::from_utf8(s[..l].to_vec()).expect("utf8")
}
  }
}

/// trait Display for XMat44F32
impl std::fmt::Display for XMat44F32 {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(17, 7))
  }
}

/// trait Debug for XMat44F32
impl std::fmt::Debug for XMat44F32 {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(17, 7))
  }
}
