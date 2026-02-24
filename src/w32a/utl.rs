//! utl.rs

use crate::core::{voidp_mut, from_voidp_mut};
use std::{ffi};
use std::os::windows::ffi::OsStrExt;
use std::{result::Result, error::Error, ptr::null_mut};
// use windows::Win32::Graphics::Gdi::HGDIOBJ; // as string in macro
use windows::{
  Data::Xml::Dom::*,
  core::*,
  Win32::UI::{WindowsAndMessaging::*},
  Win32::{Foundation::*, Security::*, System::Threading::*}
};

/// format to PCSTR ($f: fmt, $a: any)
#[macro_export]
macro_rules! fmt_s {
  ($f:literal, $a:expr) => {{
    PCSTR(a(format!($f, $a).as_str()).as_ptr())
  }};
}
pub use fmt_s;

/// format to PCSTR ($f: fmt, $a: PCSTR, $b: PCSTR)
#[macro_export]
macro_rules! fmt_s_s {
  ($f:literal, $a:expr, $b:expr) => {{
    let s = $a.to_string().expect("PCSTR");
    let t = $b.to_string().expect("PCSTR");
    PCSTR(a(format!($f, s.as_str(), t.as_str()).as_str()).as_ptr())
  }};
}
pub use fmt_s_s;

/// format to PCWSTR ($f: fmt, $a: any)
#[macro_export]
macro_rules! fmt_w {
  ($f:literal, $a:expr) => {{
    PCWSTR(l(format!($f, $a).as_str()).as_ptr())
  }};
}
pub use fmt_w;

/// format to PCWSTR ($f: fmt, $a: PCWSTR, $b: PCWSTR)
#[macro_export]
macro_rules! fmt_w_w {
  ($f:literal, $a:expr, $b:expr) => {{
    let s = $a.to_string().expect("PCWSTR");
    let t = $b.to_string().expect("PCWSTR");
    PCWSTR(l(format!($f, s.as_str(), t.as_str()).as_str()).as_ptr())
  }};
}
pub use fmt_w_w;

/// asciiz literal same as s!
#[inline]
pub fn a(s: &str) -> Vec<u8> {
  let mut v = s.bytes().collect::<Vec<_>>();
  v.push(0); // last NULL
  v
}

/// wcs z literal same as w!
#[inline]
pub fn l(s: &str) -> Vec<u16> {
  let mut v = ffi::OsStr::new(s).encode_wide().collect::<Vec<_>>();
  v.push(0); // last NULL
  v
}

/// cast_gdiobj macro unsafe
#[macro_export]
macro_rules! cast_gdiobj {
  ($t:ty, $o:expr) => { cast_any!($t, HGDIOBJ, $o) };
}
pub use cast_gdiobj;

/// cast_gdiobj_mut macro unsafe
#[macro_export]
macro_rules! cast_gdiobj_mut {
  ($t:ty, $o:expr) => { cast_any_mut!($t, HGDIOBJ, $o) };
}
pub use cast_gdiobj_mut;

/// test_utl
pub fn test_utl() -> Result<(), Box<dyn Error>> {
  let wnd_a = from_voidp_mut::<HWND>(null_mut());
  let wnd_b = HWND(null_mut());
  assert_eq!(wnd_a, wnd_b);

  let doc = XmlDocument::new()?;
  doc.LoadXml(h!("<HTML>Hello, World!</HTML>"))?;
  let rt = doc.DocumentElement()?;
  assert_eq!(rt.NodeName()?, "HTML");
  assert_eq!(rt.InnerText()?, "Hello, World!");

unsafe {
  // let opt = MB_OK | MB_ICONEXCLAMATION; // MB_OK
  // let opt = MB_OKCANCEL | MB_ICONQUESTION; // MB_OK
  let opt = MB_YESNOCANCEL | MB_ICONINFORMATION; // MB_YESNO
  assert_eq!(MessageBoxA(None,
    PCSTR(a("a msgA\n\npush YES\n\nto continue\notherwise failure").as_ptr()),
    s!("s titleA s titleA s titleA s titleA s titleA s titleA s titleA"),
    opt), IDYES);
  assert_eq!(MessageBoxW(None,
    PCWSTR(l("l msgW\n\npush YES\n\nto continue\notherwise failure").as_ptr()),
    w!("w titleW w titleW w titleW w titleW w titleW w titleW w titleW"),
    opt), IDYES);

  let _sa: SECURITY_ATTRIBUTES;
  let ev = CreateEventW(None, true, false, w!("eventW"))?;
  if ev != HANDLE(voidp_mut(0)) { // null_mut()
    SetEvent(ev)?;
    WaitForSingleObject(ev, 0);
    CloseHandle(ev)?;
  }
}

  Ok(())
}
