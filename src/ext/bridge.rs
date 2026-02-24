//! bridge.rs

mod cppbridge;
pub use cppbridge::{
  Cpname, D3DXVECTOR3, D3DXMATRIX, D3DXQUATERNION, D3DPRIMITIVETYPE,
  Cvtx, FVF_CVTX, D3DVERTEXELEMENT9, CvtxElem, Cxd, TransScreen as Cts,
  createD3D, destroyD3D, finishD3D, initD3D,
  Csa, manage_resource_n_pvec, manage_resource_ptr_mut,
  manage_resource_set_disposer,
  disposeManageResourceElements, disposeManageResource, initManageResource,
  initFont, initTexture, initTextureIndirect, readTexture, alphaTexture,
  prepareVertexBuffer, prepRectFAN, prepRectSTRIP,
  drawVT, drawChars, draw2DText,
  setLight, setCamera, drawD3D, updateD3D, rotCG,

  D3DXQuaternionNormalize, D3DXQuaternionInverse,
  D3DXQuaternionMultiply, D3DXQuaternionRotationAxis,
  D3DXQuaternionRotationMatrix, D3DXMatrixRotationQuaternion,
  D3DXMatrixIdentity, D3DXMatrixTranspose,
  D3DXMatrixDeterminant, D3DXMatrixInverse, D3DXMatrixMultiply,
  D3DXMatrixTranslation, D3DXMatrixScaling, D3DXMatrixRotationAxis,
  D3DXMatrixLookAtLH, D3DXMatrixPerspectiveFovLH,

  LOGFILE, initLog, outLog,

  timespec, _timespec_now, _timespec_to_double,
  _timespec_clear, _timespec_iszero, _timespec_isset,
  _timespec_cmp, _timespec_sub, _timespec_add,

  ref_mat44_i, ref_mat44_t, ref_mat44_s, ref_mat44_r, ref_mat44_q, ref_mat44_d,
  ref_mat44_z, ref_mat44_c, ref_mat44_v, ref_mat44_p,
  ref_mat_column_major, dump_mat_m44, dump_mat_m44_u8s, dump_mat,
  DispMatParam, disp_mat, cb_xll, cb_x, cb_ull, cb_ll, cb_u, cb_i, cb_f,

  CppBridge, CppBridge_CppBridge_destructor,
// CppBridge_new, CppBridge_get,
  gget
};

/*
// not pub (some fakes from header must be scoped only in bridge.rs)
use cppbridge::{
  HRESULT, E_FAIL, S_OK
};
*/

use crate::core::VoidpMut;
//use std::ffi;

/// test_dump;
pub fn test_dump(u8s: &mut [u8]) -> usize {
unsafe {
  dump_mat(u8s as *mut [u8] as *mut u8, u8s.len(),
    &dump_mat_m44 as *const f32, 4, 4)
}
}

/// test_gget
pub fn test_gget(u: u64) -> u64 {
  let r;
unsafe {
/**/
  let mut b = CppBridge::new(u as VoidpMut);
  r = gget(&mut b);
  CppBridge_CppBridge_destructor(&mut b);
/**/
/*
  let b = CppBridge_new(u as VoidpMut);
  r = gget(b);
  CppBridge_CppBridge_destructor(b);
*/
}
  r
}

/// Dum
pub struct Dum {
  /// bridge
  bridge: CppBridge,
//  bridge: *mut CppBridge,
  /// wnd
  wnd: u64
}

/// trait Drop for Dum
impl Drop for Dum {
  /// drop
  fn drop(&mut self) {
unsafe {
    CppBridge_CppBridge_destructor(&mut self.bridge);
//    CppBridge_CppBridge_destructor(self.bridge);
}
  }
}

/// Dum
impl Dum {
  /// new
  pub fn new(p: u64, w: u64) -> Self {
    Dum{bridge: unsafe { CppBridge::new(p as VoidpMut) }, wnd: w}
//    Dum{bridge: unsafe { CppBridge_new(p as VoidpMut) }, wnd: w}
  }

  /// get
  pub fn get(&mut self) -> u64 {
    (unsafe { self.bridge.get() }) as u64
//    (unsafe { CppBridge_get(self.bridge) }) as u64
  }

  /// wnd
  pub fn wnd(&self) -> u64 {
    self.wnd
  }
}
