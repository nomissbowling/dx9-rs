//! wm.rs

// #![allow(unused_unsafe)]

use crate::core::*;
use crate::w32a::*;
use std::{mem};
use std::{result::Result, error::Error, ptr::null_mut};
use windows::{
  core::*,
  Win32::UI::{WindowsAndMessaging::*, Input::KeyboardAndMouse::*},
  Win32::Graphics::Gdi::*,
  Win32::Media::{timeBeginPeriod, timeEndPeriod},
  Win32::System::LibraryLoader::GetModuleHandleA,
  Win32::{Foundation::*}
};

/// TransScreen
#[repr(C)]
pub struct TransScreen {
  /// LONG w
  pub w: i32,
  /// LONG h
  pub h: i32,
  /// HWND wnd
  pub wnd: HWND,
  /// HDC mdc
  pub mdc: HDC,
  /// HBITMAP bmp
  pub bmp: HBITMAP
}

/// trans_d3d
pub fn trans_d3d(dst: HWND, wnd: HWND) -> HRESULT {
unsafe {
  let ptss = GetWindowLongPtrW(dst, GWLP_USERDATA) as *mut TransScreen;
  if ptss == null_mut() { return E_FAIL; }
  let tss = &mut *ptss;
  if tss.wnd == HWND(null_mut()) { return E_FAIL; }
  let dc = GetDC(Some(wnd));
  let _ = BitBlt(tss.mdc, 0, 0, tss.w, tss.h, Some(dc), 0, 0, SRCCOPY);
  ReleaseDC(Some(wnd), dc);
  let _ = InvalidateRect(Some(dst), None, false);
}
  S_OK
}

/// wndproc
#[unsafe(no_mangle)]
pub extern "system" fn wndproc(wnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM)
  -> LRESULT {
catch_panic!(UNWIND, unsafe {
  match msg {
  WM_CREATE => {
    SetWindowLongPtrW(wnd, GWLP_USERDATA, 0);
/*
    panic!("in create"); // test
    return LRESULT(0); // unreachable
*/
  }
  WM_DESTROY => {
//    PostQuitMessage(0);
  }
  WM_CLOSE => {
    let ptss = GetWindowLongPtrW(wnd, GWLP_USERDATA) as *mut TransScreen;
    if ptss != null_mut() {
      let tss = &mut *ptss;
      tss.wnd = HWND(null_mut());
    }else{
      PostQuitMessage(0);
//      DestroyWindow(wnd).expect("wndproc Result");
    }
  }
  WM_PAINT => {
//    let _ = ValidateRect(Some(wnd), None);
    let ptss = GetWindowLongPtrW(wnd, GWLP_USERDATA) as *mut TransScreen;
    if ptss != null_mut() {
      let tss = &mut *ptss;
      let mut ps: PAINTSTRUCT = Default::default();
      let dc = BeginPaint(wnd, &mut ps);
      let _ = BitBlt(dc, 0, 0, tss.w, tss.h, Some(tss.mdc), 0, 0, SRCCOPY);
      let _ = EndPaint(wnd, &mut ps);
      return LRESULT(0);
    }
  }
  WM_KEYDOWN => {
    match cast_any!(VIRTUAL_KEY, WPARAM, wp) {
    VK_ESCAPE => {
//      panic!("ESCAPE"); // test
      let _ = PostMessageW(Some(wnd), WM_CLOSE, WPARAM(0), LPARAM(0));
    }
    _ => {}
    }
  }
  WM_SIZE => {
/*
    if wp == SIZE_RESTORED || wp == SIZE_MAXIMIZED {
      let _ = ValidateRect(Some(wnd), None);
    }
    return LRESULT(0);
*/
  }
  _ => {}
  }
//  LRESULT(1)
  DefWindowProcW(wnd, msg, wp, lp)
})
}

/// create_window
pub fn create_window(mut dx: Dx9,
  wproc: extern "system" fn (HWND, u32, WPARAM, LPARAM) -> LRESULT,
  clsname: PCWSTR, appname: PCWSTR, menuname: PCWSTR) ->
  Result<i32, Box<dyn Error>> {
unsafe {
  let sz = dx.size();
  let inst = GetModuleHandleA(None)?;
  let wc = WNDCLASSEXW {
    cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
    style: CS_HREDRAW | CS_VREDRAW,
    lpfnWndProc: Some(wproc),
    cbClsExtra: 0,
    cbWndExtra: mem::size_of::<VoidpMut>() as i32,
    hInstance: inst.into(),
    hIcon: LoadIconW(None, IDI_APPLICATION)?,
    hCursor: LoadCursorW(None, IDC_ARROW)?,
    hbrBackground: cast_gdiobj!(HBRUSH, GetStockObject(BLACK_BRUSH)),
    lpszMenuName: menuname, // null_mut()
    lpszClassName: clsname,
    hIconSm: LoadIconW(None, IDI_APPLICATION)?,
    ..Default::default()};
  let r = RegisterClassExW(&wc);
  assert!(r != 0);
  // let ws = WS_OVERLAPPEDWINDOW | WS_VISIBLE;
  let ws = WS_OVERLAPPED | WS_SYSMENU | WS_MINIMIZEBOX | WS_CAPTION;
  let mut rct = RECT{left: 0, top: 0, right: sz[0], bottom: sz[1]};
  // false: without MENU bar
  AdjustWindowRectEx(&mut rct as *mut RECT, ws, false, WINDOW_EX_STYLE(0))?;
  let (w, h) = (rct.right - rct.left, rct.bottom - rct.top);
  let wnd = CreateWindowExW(WINDOW_EX_STYLE::default(), clsname, appname, ws,
    0, 0, w, h, None, None, Some(inst.into()), None)?; // CW_USEDEFAULT
  resume_panic!(UNWIND);
  assert_ne!(wnd, HWND(null_mut()));
  let r = ShowWindow(wnd, SW_SHOW);
  assert!(r == false); // create window (false: without true: with) WS_VISIBLE
  // UpdateWindow(wnd)?;

  let mut tss = (0..4).into_iter().map(|i| {
    let winname = PCWSTR(l(format!("{}_{:04}", "kari", i).as_str()).as_ptr());
    let Ok(wnd) = CreateWindowExW(
      WINDOW_EX_STYLE::default(), clsname, winname, ws,
      512 - 40 + w * (i % 2), (h - 60) * (i / 2), w, h,
      Some(wnd), None, Some(inst.into()), None) else { panic!("sub create"); };
    resume_panic!(UNWIND);
    assert_ne!(wnd, HWND(null_mut()));
    let r = ShowWindow(wnd, SW_SHOW);
    assert!(r == false); // create window without/with WS_VISIBLE
    // UpdateWindow(wnd)?;
    let dc = GetDC(Some(wnd));
    let mdc = CreateCompatibleDC(Some(dc));
    let bmp = CreateCompatibleBitmap(dc, w, h);
    let _ = SelectObject(mdc, bmp.into());
    TransScreen{w, h, wnd, mdc, bmp}
  }).collect::<Vec<_>>();
  // separate iteration to set fixed address of tss elements
  tss.iter_mut().for_each(|t| {
    SetWindowLongPtrW(t.wnd, GWLP_USERDATA, t as *mut TransScreen as isize);
  });

  let mut result = 0i32;
  if !failed!(dx.init_d3d(wnd)) {
    if !failed!(dx.init_manage_resource(Dx9::disposer))
      && !failed!(dx.init_font())
      && !failed!(dx.init_texture()) { result = 1; }
  }
  if result != 0 {
    timeBeginPeriod(1);
    let mut msg = MSG::default();
/*
    while GetMessageW(&mut msg, None, 0, 0).into() {
      if msg.message == WM_QUIT { break; }
      let _ = TranslateMessage(&msg);
      DispatchMessageW(&msg);
      resume_panic!(UNWIND);
    }
*/
    while msg.message != WM_QUIT {
      if PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() {
        let _ = TranslateMessage(&msg);
        DispatchMessageW(&msg);
      }else{
        tss.iter_mut().for_each(|t| {
          let _ = dx.draw_d3d();
          let _ = trans_d3d(t.wnd, wnd);
        });
        let _ = dx.update_d3d();
      }
      resume_panic!(UNWIND);
    }
    result = cast_any!(i32, WPARAM, msg.wParam);
    timeEndPeriod(1);
  }
  dx.finish_d3d();
  tss.iter_mut().for_each(|t| {
    let _ = DeleteObject(t.bmp.into());
    let _ = DeleteDC(t.mdc);
  });
  UnregisterClassW(clsname, Some(wc.hInstance))?;
  Ok(result)
}
}
