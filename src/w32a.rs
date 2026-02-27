//! w32a.rs
/*
  features https://microsoft.github.io/windows-rs/features/

  DirectX COM etc https://zenn.dev/lnseab/articles/50d6b177f089bc
  DirectX12 https://zenn.dev/_username/articles/67d82a83294260

  https://www.youtube.com/watch?v=-oZrsCPKsn4
  https://learn.microsoft.com/ja-jp/windows/dev-environment/rust/rust-for-windows

  https://crates.io/crates/windows
  https://github.com/microsoft/windows-rs
  https://docs.rs/crate/windows/latest
  https://microsoft.github.io/windows-docs-rs/doc/windows/

  https://aquasoftware.net/blog/?p=2223
  https://kashiwaba-yuki.com/rust-winapi
  https://qiita.com/myhr47/items/194eac5a9619d813f214
  https://www.ne.jp/asahi/hishidama/home/tech/rust/windows/index.html
*/

// #![allow(unused_unsafe)]

pub mod utl;
pub mod wm;
pub mod unwind;

pub use utl::*;
pub use wm::*;
pub use unwind::*;

use crate::core::*;
use std::{result::Result, error::Error, ptr::null_mut};
use windows::{
  core::*,
  Win32::Graphics::Gdi::*,
  Win32::{Foundation::*}
};

/// test_app
pub fn test_app() -> Result<i32, Box<dyn Error>> {
unsafe {
  initLog(LOGFILE);
}
/*
  let stat = "./stat.log"; // same as LOGFILE
unsafe {
  outLog(LOGFILE, &l("start\n")[0]);
}
  out_log(stat, "begin window\n");
*/
  let appname = "Dx9 Test App";
  let sz: [i32; 2] = [32 * 32, 32 * 24];
  let (w, h) = (sz[0], sz[1]);
  let eps = [
    [0.0f32, 3.0f32, -2.5f32, 0.0f32], // [0.0, 2.5, 3.0, 0.0]
    [0.0f32, 0.0f32, -2.5f32, 0.0f32],
    [0.0f32, -3.0f32, -2.5f32, 0.0f32],
    [3.0f32, 0.0f32, -2.5f32, 0.0f32]];
  let tss = eps.into_iter().enumerate().map(|(u, ep)| {
    let i = u as i32;
    let la = [0.0f32, 0.0f32, 0.0f32, 0.0f32];
    let top = [0.0f32, 1.0f32, 0.0f32, 0.0f32]; // [0.0, 0.0, 1.0, 0.0]
    let (x, y) = (512 - 40 + w * (i % 2), (h - 60) * (i / 2));
    let owner = HWND(null_mut());
    let wnd = HWND(null_mut());
    let mdc = HDC(null_mut());
    let bmp = HBITMAP(null_mut());
//    let winname = fmt_w_w!("{}_{}", appname, fmt_w!("{:04}", i));
    let buf = l(format!("{}_{:04}", appname, i).as_str());
    let winname = PCWSTR(buf.as_ptr());
    TransScreen{ep, la, top, winname, x, y, w, h, owner, wnd, mdc, bmp, buf}
  }).collect::<Vec<_>>();
  let fncs = Fncs{
    render: |_spc, dx, t| {
      let _ = dx.draw_d3d(t);
      Ok(())
    },
    step: |_spc, dx| {
      let _ = dx.update_d3d();
      Ok(())
    }
  };
  let spc = Space{fncs};
  let sa: Vec<usize> = vec![4, 32, 4, 4, 4, 4, 32, 4];
  let result = create_window(Dx9::new(sz, &sa).expect("Dx9"), tss, spc,
    wndproc, w!("Dx9 Class"), PCWSTR(l(appname).as_ptr()), w!(""))?;
/*
  out_log(stat, "end window\n");
unsafe {
  outLog(LOGFILE, &l("end\n")[0]);
}
*/
  Ok(result)
}
