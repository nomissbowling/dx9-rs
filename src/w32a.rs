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
use std::{result::Result, error::Error}; // ptr::null_mut
use windows::{
  core::*
};

/// test_app
pub fn test_app(sz: [i32; 2], sa: &[usize]) -> Result<i32, Box<dyn Error>> {
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
  let result = create_window(Dx9::new(sz, sa).expect("Dx9"),
    wndproc, w!("Dx9 Class"), w!("Dx9 Test App"), w!(""))?;
/*
  out_log(stat, "end window\n");
unsafe {
  outLog(LOGFILE, &l("end\n")[0]);
}
*/
  Ok(result)
}
