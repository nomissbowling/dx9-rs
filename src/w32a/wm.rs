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

/// wndproc
#[unsafe(no_mangle)]
pub extern "system" fn wndproc(wnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM)
  -> LRESULT {
catch_panic!(UNWIND, unsafe {
  match msg {
/*
  WM_CREATE => {
    panic!("in create"); // test
    return LRESULT(0); // unreachable
  }
*/
  WM_DESTROY => {
    PostQuitMessage(0);
  }
  WM_CLOSE => {
    PostQuitMessage(0);
  }
  WM_KEYDOWN => {
    match cast_any!(VIRTUAL_KEY, WPARAM, wp) {
    VK_ESCAPE => {
//      panic!("ESCAPE"); // test
      PostQuitMessage(0);
      DestroyWindow(wnd).expect("wndproc Result");
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
  WM_PAINT => {
    let _ = ValidateRect(Some(wnd), None);
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
    cbWndExtra: 0,
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
  let wnd = CreateWindowExW(WINDOW_EX_STYLE::default(), clsname, appname, ws,
    CW_USEDEFAULT, CW_USEDEFAULT, rct.right - rct.left, rct.bottom - rct.top,
    None, None, Some(inst.into()), None)?; // None, None, None, None
  resume_panic!(UNWIND);
  assert_ne!(wnd, HWND(null_mut()));
  let r = ShowWindow(wnd, SW_SHOW);
  assert!(r == false); // create window (false: without true: with) WS_VISIBLE
  // UpdateWindow(wnd)?;

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
        let _ = dx.draw_d3d();
        let _ = dx.update_d3d();
      }
      resume_panic!(UNWIND);
    }
    result = cast_any!(i32, WPARAM, msg.wParam);
    timeEndPeriod(1);
  }
  dx.finish_d3d();
  UnregisterClassW(clsname, Some(wc.hInstance))?;
  Ok(result)
}
}
