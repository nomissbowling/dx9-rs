//! ts.rs

use crate::core::*;

/// TS64 as timespec
pub type TS64 = timespec;

/// TS64
impl TS64 {
  /// new constructor
  #[inline]
  pub fn new(tv_sec: i64, tv_nsec: i32) -> Self {
    TS64{tv_sec, tv_nsec}
  }
  /// sec part
  #[inline]
  pub fn ip(&self) -> i64 {
    self.tv_sec
  }
  /// nsec part
  #[inline]
  pub fn fp(&self) -> i32 {
    self.tv_nsec
  }
  /// now constructor
  #[inline]
  pub fn now() -> Self {
    let mut ts = TS64::new(0, 0);
    ts.set_now();
    ts
  }
  /// set now
  #[inline]
  pub fn set_now(&mut self) {
unsafe {
    _timespec_now(self.ptr_mut());
}
  }
  /// to_f64
  #[inline]
  pub fn to_f64(&self) -> f64 {
unsafe {
    _timespec_to_double(*self)
}
  }
  /// clear
  #[inline]
  pub fn clear(&mut self) {
unsafe {
    _timespec_clear(self);
}
  }
  /// iszero
  #[inline]
  pub fn iszero(&self) -> bool {
unsafe {
    _timespec_iszero(*self)
}
  }
  /// isset
  #[inline]
  pub fn isset(&self) -> bool {
unsafe {
    _timespec_isset(*self)
}
  }
  /// cmp
  #[inline]
  pub fn cmp(&self, c: &TS64) -> i32 {
unsafe {
    _timespec_cmp(*self, *c)
}
  }
  /// sub create new instance
  #[inline]
  pub fn sub(&self, c: &TS64) -> Self {
    let mut ts = TS64::new(0, 0);
unsafe {
    _timespec_sub(ts.ptr_mut(), *self, *c);
}
    ts
  }
  /// add create new instance
  #[inline]
  pub fn add(&self, c: &TS64) -> Self {
    let mut ts = TS64::new(0, 0);
unsafe {
    _timespec_add(ts.ptr_mut(), *self, *c);
}
    ts
  }
}

/// trait Ptr for TS64
impl Ptr<timespec> for TS64 {
  /// ptr
  fn ptr(&self) -> *const timespec { self as *const TS64 as *const timespec }
  /// ptr_mut
  fn ptr_mut(&mut self) -> *mut timespec { self as *mut TS64 as *mut timespec }
}

/// trait Disp for TS64
impl Disp for TS64 {
  /// disp
  fn disp(&self, di: usize, df: usize) -> String {
    format!("{:di$}.{:0df$}", self.tv_sec, self.tv_nsec)
  }
}

/// trait Display for TS64
impl std::fmt::Display for TS64 {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(14, 9))
  }
}

/* conflicting implementation for `timespec` #[derive(Debug, Copy, Clone)]
/// trait Debug for TS64
impl std::fmt::Debug for TS64 {
  /// fmt
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.disp(14, 9))
  }
}
*/

/// test_timespec
pub fn test_timespec() {
  let ts_s = TS64::now();
  assert_eq!(TS64::new(0, 0).iszero(), true);
  let mut ts = TS64::now();
  assert_eq!(ts.isset(), true);
  ts.clear();
  assert_eq!(ts.iszero(), true);
  assert_eq!(ts.cmp(&TS64::now()) < 0, true);
  assert_eq!(TS64::now().cmp(&ts) > 0, true);
  assert_eq!(ts.cmp(&ts) == 0, true);
  let ts_e = TS64::now();
  let c = ts_s.cmp(&ts_e);
  assert_eq!(c < 0, true);
  assert_eq!(c > 0, false);
  assert_eq!(c == 0, false); // cpu is slow :-p
//  assert_eq!(TS64::now().cmp(&TS64::now()) < 0, false); // God knows

  let a = TS64::new(1, 999999999);
  let b = TS64::new(0, 1);
  assert_eq!(a.sub(&b).cmp(&TS64::new(1, 999999998)), 0);
  assert_eq!(b.sub(&a).cmp(&TS64::new(0, 0)) < 0, true); // below 0
  assert_eq!(b.sub(&a).cmp(&TS64::new(-2, 2)), 0); // ok but critical
  assert_eq!(a.sub(&a).cmp(&TS64::new(0, 0)), 0);
  assert_eq!(b.sub(&b).cmp(&TS64::new(0, 0)), 0);
  assert_eq!(a.add(&b).cmp(&TS64::new(2, 0)), 0);
  assert_eq!(b.add(&a).cmp(&TS64::new(2, 0)), 0);
  assert_eq!(a.add(&a).cmp(&TS64::new(3, 999999998)), 0);
  assert_eq!(b.add(&b).cmp(&TS64::new(0, 2)), 0);

  let ts = TS64::now();
  let s = format!("{}", ts);
  let f = format!("{:24.9}", ts.to_f64());
  assert_eq!(s.len(), f.len());
  let d = s.len() - 5; // -5: 100usec, -4: 10usec, -3: 1usec
  assert_eq!(s.as_bytes()[..d], f.as_bytes()[..d]);
}
