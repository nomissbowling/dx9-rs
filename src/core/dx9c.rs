//! dx9c.rs

use crate::core::*;
use crate::w32a::*;
use std::{ffi, mem};
use std::{result::Result, error::Error, ptr::null_mut};
use windows::{
  core::*,
  Win32::Graphics::Gdi::*,
  Win32::{Foundation::*}
};

/// MARKER_DX9C
pub const MARKER_DX9C: &[u8; 4] = b"DX9c";

/// as_dx9c macro unsafe
#[macro_export]
macro_rules! as_dx9c {
  ($o:expr) => {
unsafe {
    assert_ne!($o, null_mut());
//    let owner = &std::slice::from_raw_parts($o as *const Dx9, 1)[0];
    let owner = &*($o as *const Dx9);
    assert_eq!(owner.mk, *MARKER_DX9C);
    owner
}
  };
}
pub use as_dx9c;

/// as_dx9c_mut macro unsafe
#[macro_export]
macro_rules! as_dx9c_mut {
  ($o:expr) => {
unsafe {
    assert_ne!($o, null_mut());
//    let owner = &mut std::slice::from_raw_parts_mut($o as *mut Dx9, 1)[0];
    let owner = &mut *($o as *mut Dx9);
    assert_eq!(owner.mk, *MARKER_DX9C);
    owner
}
  };
}
pub use as_dx9c_mut;

/// ResourceManager
pub struct ResourceManager {
  /// size array
  pub sa: Csa,
  /// vec size to be allocated
  pub vs: Vec<usize>,
  /// pvecs
  pub pvs: Vec<Pvec>
}

/// ResourceManager
impl ResourceManager {
  /// new
  pub fn new(xd: *mut Cxd, sza: &[usize]) -> Result<Self, Box<dyn Error>> {
    let n_pvec = unsafe { manage_resource_n_pvec(xd) };
    let vs = sza.to_vec(); // copy
    assert_eq!(n_pvec, vs.len());
    let pvs = (0..n_pvec).into_iter().map(|i|
      Pvec::new(vec![null_mut(); vs[i]])).collect::<Vec<_>>();
    let sa = Csa{a: &vs[0], n: vs.len()}; // to keep lifetime in Csa::a
    Ok(ResourceManager{sa, vs, pvs})
  }

  /// probe
  pub fn probe(&self) -> u64 {
    println!("n_pvecs: {}", self.pvs.len());
    self.pvs.iter().enumerate().for_each(|(i, pv)|
      println!("{:3}{:3}{}", i, pv.v.len(), pv));
    1
  }

  /// mr_probe ***void *o to *mut Dx9***
  #[unsafe(no_mangle)]
  pub extern "C" fn mr_probe(o: VoidpMut) -> ffi::c_ulonglong {
    if o == null_mut() { 0 } else { as_dx9c_mut!(o).rm.probe() }
  }
}

/// Dx9
pub struct Dx9 {
  /// marker
  mk: [u8; 4],
  /// resource manager
  rm: ResourceManager,
  /// sz [i32; 2] (xd has w, h, too)
  sz: [i32; 2],
  /// xd *mut Cxd
  xd: *mut Cxd
}

/// trait Drop for Dx9
impl Drop for Dx9 {
  /// drop
  fn drop(&mut self) {
unsafe {
    destroyD3D(&mut self.xd);
}
  }
}

/// Dx9
impl Dx9 {
  /// new
  pub fn new(sz: [i32; 2], sa: &[usize]) -> Result<Self, Box<dyn Error>> {
    if sa[1] < 32 { return Err("sa[1] must be >= 32".into()); }
    if sa[6] < 32 { return Err("sa[6] must be >= 32".into()); }
    let xd = unsafe { createD3D() };
    let rm = ResourceManager::new(xd, sa)?;
    if xd == null_mut() {
      Err("null Cxd".into())
    }else{
      Ok(Dx9{mk: *MARKER_DX9C, rm, sz, xd})
    }
  }

  /// size
  pub fn size(&self) -> &[i32; 2] {
    &self.sz
  }

  /// finish_d3d
  pub fn finish_d3d(&self) {
unsafe {
    finishD3D(self.xd);
}
  }

  /// init_d3d
  pub fn init_d3d(&mut self, wnd: HWND) -> HRESULT { // mut wnd: HWND
unsafe {
    HRESULT(initD3D(self.xd, voidp_mut(wnd)))
}
  }

  /// dispose_manage_resource_elements
  pub fn dispose_manage_resource_elements(&mut self) -> HRESULT {
unsafe {
    HRESULT(disposeManageResourceElements(self.xd))
}
  }

  /// dispose_manage_resource
  pub fn dispose_manage_resource(&mut self) -> HRESULT {
self.rm.probe();
    if failed!(self.dispose_manage_resource_elements()) { return E_FAIL; }
unsafe {
    // no care self.rm.pvs (pvecs will be auto free)
    let n = manage_resource_n_pvec(self.xd);
    std::slice::from_raw_parts_mut(manage_resource_ptr_mut(self.xd), n)
      .iter_mut().for_each(|q| *q = null_mut());
}
    S_OK
/* // use c function
unsafe {
    let o = self as *mut Self as VoidpMut; // not same as cast_any_mut!
    HRESULT(disposeManageResource(self.xd, o))
}
*/
  }

  /// disposer ***void *o to *mut Dx9***
  #[unsafe(no_mangle)]
  pub extern "C" fn disposer(xd: *mut Cxd, o: VoidpMut) -> ffi::c_long {
    let owner = as_dx9c_mut!(o);
    assert_eq!(xd, owner.xd);
unsafe {
    cast_any!(ffi::c_long, HRESULT, owner.dispose_manage_resource())
}
  }

  /// init_manage_resource
  pub fn init_manage_resource(&mut self,
    d: unsafe extern "C" fn (xd: *mut Cxd, o: VoidpMut) -> ffi::c_long) ->
    HRESULT {
unsafe {
    let o = self as *mut Self as VoidpMut; // not same as cast_any_mut!
    manage_resource_set_disposer(self.xd, o, Some(d), &self.rm.sa);
    let n = manage_resource_n_pvec(self.xd);
    std::slice::from_raw_parts_mut(manage_resource_ptr_mut(self.xd), n)
      .iter_mut().enumerate().for_each(|(i, q)| {
        let pv = &mut self.rm.pvs[i];
        *q = if pv.v.len() == 0 { null_mut() } else { pv.v.as_mut_ptr() }});
}
    S_OK
/* // use c function
unsafe {
    let o = self as *mut Self as VoidpMut; // not same as cast_any_mut!
    HRESULT(initManageResource(self.xd, o, Some(d), &self.rm.sa))
}
*/
  }

  /// init_font
  pub fn init_font(&mut self) -> HRESULT {
unsafe {
    HRESULT(initFont(self.xd))
}
  }

  /// init_texture
  pub fn init_texture(&mut self) -> HRESULT {
unsafe {
    HRESULT(initTexture(self.xd))
}
  }

  /// init_texture_indirect
  pub fn init_texture_indirect(&mut self,
    n: usize, w: u32, h: u32, q: &mut u32) -> HRESULT {
unsafe {
    HRESULT(initTextureIndirect(self.xd, n, w, h, q as *mut u32))
}
  }

  /// read_texture
  pub fn read_texture(&mut self,
    n: usize, w: u32, h: u32, q: &mut u32) -> HRESULT {
unsafe {
    HRESULT(readTexture(self.xd, n, w, h, q as *mut u32))
}
  }

  /// alpha_texture
  pub fn alpha_texture(&mut self,
    n: usize, w: u32, h: u32, mask: u32) -> HRESULT {
unsafe {
    HRESULT(alphaTexture(self.xd, n, w, h, mask))
}
  }

  /// prepare_vertex_buffer
  pub fn prepare_vertex_buffer(&mut self,
    n: u32, vtx: *mut Cvtx, sz: u32, fvf: u32) -> HRESULT {
unsafe {
    HRESULT(prepareVertexBuffer(self.xd, n, vtx, sz, fvf))
}
  }

  /// prep_rect_fan Cvtx[4]
  pub fn prep_rect_fan(&mut self, vtx: *mut Cvtx, c: *mut u32, s: *mut u32,
    u: f32, v: f32, w: f32, h: f32,
    x: f32, y: f32, z: f32, a: f32, b: f32, cg: *mut D3DXVECTOR3) -> HRESULT {
unsafe {
    HRESULT(prepRectFAN(self.xd, vtx, c, s, u, v, w, h, x, y, z, a, b, cg))
}
  }

  /// prep_rect_strip Cvtx[4]
  pub fn prep_rect_strip(&mut self, vtx: *mut Cvtx, c: *mut u32, s: *mut u32,
    u: f32, v: f32, w: f32, h: f32,
    x: f32, y: f32, z: f32, a: f32, b: f32, cg: *mut D3DXVECTOR3) -> HRESULT {
unsafe {
    HRESULT(prepRectSTRIP(self.xd, vtx, c, s, u, v, w, h, x, y, z, a, b, cg))
}
  }

  /// draw_vt st: start, pc: primitive count
  pub fn draw_vt(&mut self, t: u32, v: u32, vtx: *mut Cvtx, sz: u32, fvf: u32,
    ptype: D3DPRIMITIVETYPE, st: u32, pc: u32) -> HRESULT {
unsafe {
    HRESULT(drawVT(self.xd, t, v, vtx, sz, fvf, ptype, st, pc))
}
  }

  /// draw_chars texture cell w/h scale w/h
  pub fn draw_chars(&mut self, c: *mut u32, s: *mut u32,
    t: u32, cw: i32, ch: i32, sw: i32, sh: i32, x: f32, y: f32, z: f32,
    w: &str) -> HRESULT {
unsafe {
    let sz = w.len() as u32;
    HRESULT(drawChars(self.xd, c, s, t, cw, ch, sw, sh, x, y, z, &l(w)[0], sz))
}
  }

  /// draw_2d_text
  pub fn draw_2d_text(&mut self,
    c: u32, f: u32, x: i32, y: i32, t: &str) -> HRESULT {
unsafe {
    HRESULT(draw2DText(self.xd, c, f, x, y, &l(t)[0]))
}
  }

  /// set_light
  pub fn set_light(&mut self) -> HRESULT {
unsafe {
    HRESULT(setLight(self.xd))
}
  }

  /// set_camera
  pub fn set_camera(&mut self, tss: &mut TransScreen) -> HRESULT {
unsafe {
    HRESULT(setCamera(self.xd, tss as *mut TransScreen as *mut Cts))
}
  }

  /// draw_d3d
  pub fn draw_d3d(&mut self, tss: &mut TransScreen) -> HRESULT {
unsafe {
    HRESULT(drawD3D(self.xd, tss as *mut TransScreen as *mut Cts))
}
  }

  /// update_d3d
  pub fn update_d3d(&mut self) -> HRESULT {
unsafe {
    HRESULT(updateD3D(self.xd))
}
  }
}

/// test_dx9c
pub fn test_dx9c() {
  assert_eq!(mem::size_of::<*mut ffi::c_void>(), 8);
  assert_eq!(mem::size_of::<HWND>(), 8);
  assert_eq!(mem::size_of::<HANDLE>(), 8);
  assert_eq!(mem::size_of::<HGDIOBJ>(), 8);
  assert_eq!(mem::size_of::<HBRUSH>(), 8);
  assert_eq!(mem::size_of::<HRESULT>(), 4);
  assert_eq!(format!("{:08x}", unsafe { cast_any!(i32, HRESULT, E_FAIL) }),
    "80004005");
  assert_eq!(format!("{:08x}", unsafe { cast_any!(i32, HRESULT, S_OK) }),
    "00000000");
//  assert_eq!(mem::size_of::<D3DPRESENT_PARAMETERS>(), 64); // hidden
  assert_eq!(mem::size_of::<Cxd>(), 0); // unknown hidden
  assert_eq!(mem::size_of::<Cvtx>(), 28);
  assert_eq!(format!("{:08x}", unsafe { FVF_CVTX }), "000001c2");
  assert_eq!(mem::size_of::<D3DVERTEXELEMENT9>(), 0); // unknown hidden (or 8)
  assert_eq!(mem::size_of::<timespec>(), 16);
}
