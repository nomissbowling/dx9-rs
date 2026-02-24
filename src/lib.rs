#![doc(html_root_url = "https://docs.rs/dx9/0.1.2")]
//! Rust library for DirectX 9.0c
//!
//! # Requirements
//!
//! - [DirectX Software Development Kit (DXSDK_Jun10.exe)](https://www.microsoft.com/en-us/download/details.aspx?id=6812)
//!

pub mod w32a;
pub mod core;
pub mod ext;

/// with [-- --nocapture] or with [-- --show-output]
#[cfg(test)]
mod tests {
  use std::{result::Result, error::Error};
  use super::{*, w32a::*, core::*, ext};

  /// timespec
  #[test]
  fn test_timespec() {
    core::ts::test_timespec();
  }

  /// handle size, struct size, etc
  #[test]
  fn test_dx9c() {
    core::dx9c::test_dx9c();
  }

  /// handle, xml, messagebox, event, etc
  #[test]
  fn test_w32a_utl() {
    assert!(test_utl().expect("utl Result") == ());
  }

  /// create_window
  #[test]
  fn test_w32a() {
    let sa: Vec<usize> = vec![4, 32, 4, 4, 4, 4, 32, 4];
    assert!(test_app([32 * 32, 32 * 24], &sa).expect("app Result") == 0);
  }

  /// device manager Dx9 has handle Cxd
  #[test]
  fn test_dx9() -> Result<(), Box<dyn Error>> {
    let sa: Vec<usize> = vec![0, 32, 0, 0, 0, 0, 32, 0];
    assert_eq!(Dx9::new([800, 600], &sa)?.size(), &[800, 600]);
    Ok(())
  }

  /// dump, disp of XV*
  #[test]
  fn test_dx9_prim_xvec() -> Result<(), Box<dyn Error>> {
    let ullm = 0xffffffffffffffffu64; // 18446744073709551616 - 1
    let v2ull = XV2ULL([ullm, 0u64].map(|i| i as u64));
    assert_eq!(v2ull.dump(), "[[ ffffffffffffffff 0000000000000000]]");
    assert_eq!(v2ull.disp(21, 0), "\
[[  18446744073709551615                     0]]");
    assert_eq!(format!("{}", v2ull), "\
[[  18446744073709551615                     0]]");
    assert_eq!(format!("{:?}", v2ull), "\
[[  18446744073709551615                     0]]");

    let llm = 0x7fffffffffffffffi64; // 9223372036854775808 - 1
    let v2ll = XV2LL([llm, -llm-1].map(|i| i as i64));
    assert_eq!(v2ll.dump(), "[[ 7fffffffffffffff 8000000000000000]]");
    assert_eq!(v2ll.disp(21, 0), "\
[[   9223372036854775807  -9223372036854775808]]");
    assert_eq!(format!("{}", v2ll), "\
[[   9223372036854775807  -9223372036854775808]]");
    assert_eq!(format!("{:?}", v2ll), "\
[[   9223372036854775807  -9223372036854775808]]");

    let um = 0xffffffffu32; // 4294967296 - 1
    let v2u = XV2U([um, 0u32].map(|i| i as u32));
    assert_eq!(v2u.dump(), "[[ ffffffff 00000000]]");
    assert_eq!(v2u.disp(11, 0), "[[  4294967295           0]]");
    assert_eq!(format!("{}", v2u), "\
[[        4294967295                 0]]");
    assert_eq!(format!("{:?}", v2u), "\
[[        4294967295                 0]]");

    let im = 0x7fffffffi32; // 2147483648 - 1
    let v2i = XV2I([im, -im-1].map(|i| i as i32));
    assert_eq!(v2i.dump(), "[[ 7fffffff 80000000]]");
    assert_eq!(v2i.disp(11, 0), "[[  2147483647 -2147483648]]");
    assert_eq!(format!("{}", v2i), "\
[[        2147483647       -2147483648]]");
    assert_eq!(format!("{:?}", v2i), "\
[[        2147483647       -2147483648]]");

    let v2f = XV2F32([1, 2].map(|i| i as f32));
    assert_eq!(v2f.dump(), "[[ 3f800000 40000000]]");
    assert_eq!(v2f.disp(4, 1), "[[  1.0  2.0]]");
    assert_eq!(format!("{}", v2f), "\
[[         1.0000000         2.0000000]]");
    assert_eq!(format!("{:?}", v2f), "\
[[         1.0000000         2.0000000]]");

    let v3f = XV3F32([1, 2, 3].map(|i| i as f32));
    assert_eq!(v3f.dump(), "[[ 3f800000 40000000 40400000]]");
    assert_eq!(v3f.disp(4, 1), "[[  1.0  2.0  3.0]]");
    assert_eq!(format!("{}", v3f), "\
[[         1.0000000         2.0000000         3.0000000]]");
    assert_eq!(format!("{:?}", v3f), "\
[[         1.0000000         2.0000000         3.0000000]]");

    let v4f = XV4F32([1, 2, 3, 4].map(|i| i as f32));
    assert_eq!(v4f.dump(), "[[ 3f800000 40000000 40400000 40800000]]");
    assert_eq!(v4f.disp(4, 1), "[[  1.0  2.0  3.0  4.0]]");
    assert_eq!(format!("{}", v4f), "\
[[         1.0000000         2.0000000         3.0000000         4.0000000]]");
    assert_eq!(format!("{:?}", v4f), "\
[[         1.0000000         2.0000000         3.0000000         4.0000000]]");

    let prec = 0.000001f32;

    // test dot cross norm
    assert!(prec_eq(XV3F32([1.0, 2.0, 3.0]).dot(
      &XV3F32([6.0, -5.0, 4.0])), 8.0, prec));
    assert!(XV3F32([1.0, 2.0, 3.0]).cross(
      &XV3F32([6.0, -5.0, 4.0])).prec_eq(
        &XV3F32([23.0, 14.0, -17.0]), prec)?);
    assert!(XV3F32([1.0, 0.0, 0.0]).norm(
      &XV3F32([0.0, 1.0, 0.0]))?.prec_eq(
        &XV3F32([0.0, 0.0, 1.0]), prec)?);

    // test dot cross3 norm3
    assert!(prec_eq(XV4F32([1.0, 2.0, 3.0, 4.0]).dot(
      &XV4F32([6.0, -5.0, 4.0, -3.0])), -4.0, prec));
    assert!(XV4F32([1.0, 2.0, 3.0, 4.0]).cross3(
      &XV4F32([6.0, -5.0, 4.0, -3.0])).prec_eq(
        &XV4F32([23.0, 14.0, -17.0, 1.0]), prec)?);
    assert!(XV4F32([1.0, 0.0, 0.0, 1.0]).norm3(
      &XV4F32([0.0, 1.0, 0.0, 1.0]))?.prec_eq(
        &XV4F32([0.0, 0.0, 1.0, 1.0]), prec)?);
    Ok(())
  }

  /// dump, disp of XMat44F32
  #[test]
  fn test_dx9_prim_xmat() -> Result<(), Box<dyn Error>> {
    let m44f = XMat44F32([[1,2,3,4],[5,6,7,8],[9,10,11,12],[13,14,15,16]]
      .map(|v| v.map(|i| i as f32)));
    assert_eq!(m44f.dump(), "\
[[ 3f800000 40000000 40400000 40800000]
 [ 40a00000 40c00000 40e00000 41000000]
 [ 41100000 41200000 41300000 41400000]
 [ 41500000 41600000 41700000 41800000]]");
    assert_eq!(m44f.disp(5, 2), "\
[[  1.00  2.00  3.00  4.00]
 [  5.00  6.00  7.00  8.00]
 [  9.00 10.00 11.00 12.00]
 [ 13.00 14.00 15.00 16.00]]");
    assert_eq!(format!("{}", m44f), "\
[[         1.0000000         2.0000000         3.0000000         4.0000000]
 [         5.0000000         6.0000000         7.0000000         8.0000000]
 [         9.0000000        10.0000000        11.0000000        12.0000000]
 [        13.0000000        14.0000000        15.0000000        16.0000000]]");
    assert_eq!(format!("{:?}", m44f), "\
[[         1.0000000         2.0000000         3.0000000         4.0000000]
 [         5.0000000         6.0000000         7.0000000         8.0000000]
 [         9.0000000        10.0000000        11.0000000        12.0000000]
 [        13.0000000        14.0000000        15.0000000        16.0000000]]");
    Ok(())
  }

  /// functions of XMat44F32 prec_eq c_array_transposed
  /// static_mut_refs to avoid 'ref_mat44_* shared reference to mutable static'
  #[allow(static_mut_refs)]
  #[test]
  fn test_dx9_prim_xmat_c_array_transposed() -> Result<(), Box<dyn Error>> {
    let prec = 0.000001f32;
    let a = 0.0000023f32;
    let b = 0.0000014f32;
    assert_eq!(prec_diff(a, b, prec), 0.0f32);
    assert_eq!(prec_diff(b, a, prec), 0.0f32);

    let i_disp = "\
[[  1.00  0.00  0.00  0.00]
 [  0.00  1.00  0.00  0.00]
 [  0.00  0.00  1.00  0.00]
 [  0.00  0.00  0.00  1.00]]";

    let i = XMat44F32::i()?;
    assert_eq!(i.disp(5, 2), i_disp);
    let mut m = XMat44F32::o();
    assert!(prec_eq_c_array_transposed!(m.set_i()?, ref_mat44_i, prec)?);
    assert_eq!(m.disp(5, 2), i_disp);
    assert!(prec_eq_c_array_transposed!(m.mul(&i)?, ref_mat44_i, prec)?);
    assert_eq!(m.mul(&i)?.disp(5, 2), i_disp);
    assert!(prec_eq_c_array_transposed!(m.set_mul(&i)?, ref_mat44_i, prec)?);
    assert_eq!(m.disp(5, 2), i_disp);
    assert!(prec_eq_c_array_transposed!(m.set_from(&i), ref_mat44_i, prec)?);
    assert_eq!(m.disp(5, 2), i_disp);
    assert!(prec_eq_c_array_transposed!(XMat44F32::translation(7.0, 5.0, 3.0)?,
      ref_mat44_t, prec)?);
    assert!(prec_eq_c_array_transposed!(m.set_translation(7.0, 5.0, 3.0)?,
      ref_mat44_t, prec)?);
    assert!(prec_eq_c_array_transposed!(XMat44F32::scaling(7.0, 5.0, 3.0)?,
      ref_mat44_s, prec)?);
    assert!(prec_eq_c_array_transposed!(m.set_scaling(7.0, 5.0, 3.0)?,
      ref_mat44_s, prec)?);
    assert!(prec_eq_c_array_transposed!(
      XMat44F32::rotation_axis(&XV3F32([1.0, 0.0, 0.0]), rad(90.0))?,
      ref_mat44_r, prec)?);
    assert!(prec_eq_c_array_transposed!(
      m.set_rotation_axis(&XV3F32([1.0, 0.0, 0.0]), rad(90.0))?,
      ref_mat44_r, prec)?);

    let axis_z = XV3F32([0.0, 0.0, 1.0]);
    let rot = XMat44F32::rotation_axis(&axis_z, rad(6.0))?;
    assert!(prec_eq_c_array_transposed!(rot, ref_mat44_z, prec)?);
    let eye = XV3F32([0.0, 3.0, -2.5]);
    let lookat = XV3F32([0.0, 0.0, 0.0]);
    let up = XV3F32([0.0, 1.0, 0.0]);
    let cam = XMat44F32::look_at_lh(&eye, &lookat, &up)?;
    assert!(prec_eq_c_array_transposed!(cam, ref_mat44_c, prec)?);
    let view = rot.mul(&cam)?;
    assert!(prec_eq_c_array_transposed!(view, ref_mat44_v, prec)?);
    let prj = XMat44F32::perspective_fov_lh(rad(120.0), 4f32/3.0, 0.1, 500.0)?;
    assert!(prec_eq_c_array_transposed!(prj, ref_mat44_p, prec)?);

    // test inverse get det
    m.set_quaternion(&XV3F32([1.0, 0.0, 0.0]), rad(-90.0))?;
    let mut det = 0.0f32;
    assert!(prec_eq_c_array_transposed!(
      m.inverse(Some(&mut det))?, ref_mat44_r, prec)?);
    assert!(prec_eq(det, 1.0f32, prec));
    det = 0.0f32;
    assert!(prec_eq_c_array_transposed!(
      m.set_inverse(Some(&mut det))?, ref_mat44_r, prec)?);
    assert!(prec_eq(det, 1.0f32, prec));

    // test det and inverse
    m = XMat44F32::from_float_m04(unsafe { &ref_mat44_d });
    det = m.det(prec)?;
    assert!(prec_eq(det, -1.0f32, prec));
//    println!("{}", m);
    det = 0.0f32;
    let n = m.inverse(Some(&mut det))?;
    assert!(prec_eq(det, -1.0f32, prec));
//    println!("{}", m);
    assert!(n.mul(&m)?.prec_eq(&i, prec)?);
    assert!(n.prec_eq(&m, prec)?);

    // test transpose
    assert!(prec_eq_c_array_transposed!(
      XMat44F32::from_float_m0(unsafe { &ref_mat_column_major }),
      *(&dump_mat_m44 as *const [f32; 0] as *const [[f32; 4]; 0]), prec)?);
    Ok(())
  }

  /// functions of XMat44F32 as quaternion
  /// static_mut_refs to avoid 'ref_mat44_* shared reference to mutable static'
  #[allow(static_mut_refs)]
  #[test]
  fn test_dx9_prim_xmat_as_quaternion() -> Result<(), Box<dyn Error>> {
    let prec = 0.000001f32;
    let i = XMat44F32::i()?;
    let j = XV4F32::i();
    let axis_x = XV3F32([1.0, 0.0, 0.0]);
    let axis_y = XV3F32([0.0, 1.0, 0.0]);
    let axis_z = XV3F32([0.0, 0.0, 1.0]);
    let mut q = XMat44F32::quaternion(&axis_x, rad(-90.0))?;

    // test quaternion
    assert!(prec_eq_c_array_transposed!(q, ref_mat44_q, prec)?);

    // test transpose quaternion
    assert!(prec_eq_c_array!(q, ref_mat44_r, prec)?);
    assert!(prec_eq_c_array!(q.set_transpose()?, ref_mat44_q, prec)?);

    // test inverse quaternion
    assert!(prec_eq_c_array_transposed!(
      q.inverse(None)?, ref_mat44_q, prec)?);
    assert!(prec_eq_c_array_transposed!(
      q.set_inverse(None)?, ref_mat44_q, prec)?);

    // test quaternion conjugate
    let th = rad(90.0f32);
    let v = XV4F32::quaternion(&axis_z, th)?; // 0.0, 0.0, r2 / 2.0, r2 / 2.0
    let mut w = XV4F32::quaternion(&axis_z, th)?; // will be assigned later
    let (c, s) = (f32::cos(th / 2.0), f32::sin(th / 2.0));
    assert!(v.prec_eq(&XV4F32([0.0, 0.0, s, c]), prec)?);
    assert!(v.conjugate()?.prec_eq(&XV4F32([0.0, 0.0, -s, c]), prec)?);
    assert!(v.normalize()?.prec_eq(&v, prec)?); // normalize quad elements
    assert!(w.set_normalize()?.prec_eq(&v, prec)?); // normalize quad elements
    assert!(v.inverse()?.prec_eq(&v.conjugate()?, prec)?);

    let r = XV4F32::rotation_axis(&axis_z, th)?;
    assert!(v.prec_eq(&r, prec)?);
    assert!(v.prec_eq(XV4F32::o().set_rotation_axis(&axis_z, th)?, prec)?);

    assert!(XMat44F32::try_from(v.clone())?.prec_eq(
      &XMat44F32::try_from(r.clone())?, prec)?);
    assert!(XMat44F32::quaternion(&axis_z, th)?.prec_eq(
      &XMat44F32::try_from(r.clone())?, prec)?);

    // test to avoid divide by 0
    let _ = [
      (&axis_x, 180.0f32, &XV4F32([1.0, 0.0, 0.0, 0.0])), // cos=0
      (&axis_y, 180.0f32, &XV4F32([0.0, 1.0, 0.0, 0.0])), // cos=0 xsin=0
      (&axis_z, 180.0f32, &XV4F32([0.0, 0.0, 1.0, 0.0]))] // cos=0 xsin=ysin=0
    .into_iter().map(|(axis, th, t)| (
      |axis: &XV3F32, th: f32, t: &XV4F32| -> Result<(), Box<dyn Error>> {
      let r = XV4F32::quaternion(&axis, rad(th))?; // cos=0, sin=1
      assert!(r.prec_eq(&t, prec)?);
      let f = XV4F32::quaternion_m(&XMat44F32::try_from(r.clone())?, prec)?;
      let g = XV4F32::try_from(XMat44F32::try_from(r.clone())?)?;
      assert!(f.prec_eq(&g, prec)?);
      assert!(f.prec_eq(&r, prec)?); // f(PI) == r(PI)
      assert!(g.prec_eq(&r, prec)?); // g(PI) == r(PI)
      let s = XV4F32::quaternion(&axis, rad(-th))?; // cos=0, sin=-1
      assert!(s.prec_eq(&t.conjugate()?, prec)?); // negative
      let f = XV4F32::quaternion_m(&XMat44F32::try_from(s.clone())?, prec)?;
      let g = XV4F32::try_from(XMat44F32::try_from(s.clone())?)?;
      assert!(f.prec_eq(&g, prec)?);
      assert!(f.prec_eq(&s.conjugate()?, prec)?); // f(-PI) == ~s(-PI) negative
      assert!(g.prec_eq(&s.conjugate()?, prec)?); // g(-PI) == ~s(-PI) negative
      assert!(f.prec_eq(&r, prec)?); // f(-PI) == r(PI)
      assert!(g.prec_eq(&r, prec)?); // g(-PI) == r(PI)
      Ok(())
    })(axis, th, t)).collect::<Vec<_>>();

    // identity
    let r = XV4F32::quaternion(&axis_z, rad(0.0f32))?; // cos=1, sin=0
    assert!(r.prec_eq(&j, prec)?);

    // negative cos(th/2)
    let r = XV4F32::quaternion(&axis_z, rad(360.0f32))?; // cos=-1, sin=0
    assert!(r.prec_eq(&XV4F32([0.0, 0.0, 0.0, -1.0]), prec)?);
    let r = XV4F32::rotation_axis(&axis_z, rad(360.0f32))?; // cos=-1, sin=0
    assert!(r.prec_eq(&XV4F32([0.0, 0.0, 0.0, -1.0]), prec)?);

    // test normalize rad (always positive cos(th/2))
    let t = XV4F32([0.0, 0.0, 1.0, 0.0]); // cos=0, sin=+-1
    let h = XV4F32([0.0, 0.0, 0.707107, 0.707107]); // cos=1/r2, sin=+-1/r2
    let _ = [
      (&axis_z, rad(normalize_deg(180.0f32)), &t),
      (&axis_z, normalize_rad(rad(180.0f32)), &t),
      (&axis_z, rad(normalize_deg(-180.0f32)), &t.conjugate()?),
      (&axis_z, normalize_rad(rad(-180.0f32)), &t.conjugate()?),
      (&axis_z, rad(normalize_deg(270.0f32)), &h.conjugate()?),
      (&axis_z, normalize_rad(rad(270.0f32)), &h.conjugate()?),
      (&axis_z, rad(normalize_deg(-270.0f32)), &h),
      (&axis_z, normalize_rad(rad(-270.0f32)), &h),
      (&axis_z, rad(normalize_deg(360.0f32)), &j),
      (&axis_z, normalize_rad(rad(360.0f32)), &j),
      (&axis_z, rad(normalize_deg(-360.0f32)), &j),
      (&axis_z, normalize_rad(rad(-360.0f32)), &j)]
    .into_iter().map(|(axis, th, t)| (
      |axis: &XV3F32, th: f32, t: &XV4F32| -> Result<(), Box<dyn Error>> {
      assert!(XV4F32::quaternion(&axis, th)?.prec_eq(&t, prec)?);
      Ok(())
    })(axis, th, t)).collect::<Vec<_>>();

    let axis = XV3F32([-1.0, -1.0, -1.0]);
    let f = XV4F32::quaternion_m(
      &XMat44F32::quaternion(&axis, rad(90.0))?, prec)?;
    let g = XV4F32::try_from(
      XMat44F32::rotation_axis(&axis, rad(90.0))?)?;
    assert!(f.prec_eq(&g, prec)?);
//    assert!(f.normalize()?.prec_eq(&g.normalize()?, prec)?); // quad elements

    let axis = XV3F32([1.0, 1.0, 1.0]);
    let f = XV4F32::quaternion_m(
      &XMat44F32::quaternion(&axis, rad(-90.0))?, prec)?;
    assert!(f.prec_eq(&g, prec)?); // reverse axis reverse angle
//    assert!(f.normalize()?.prec_eq(&g.normalize()?, prec)?); // quad elements
    let g = XV4F32::try_from(
      XMat44F32::rotation_axis(&axis, rad(-90.0))?)?;
    assert!(f.prec_eq(&g, prec)?);
//    assert!(f.normalize()?.prec_eq(&g.normalize()?, prec)?); // quad elements

    let m = XMat44F32::quaternion(&axis_z, th)?; // axis angle same as v
// println!("{}", m); // 0100 -1000 0010 0001
    let u = XV4F32::quaternion_m(&m, prec)?;
    assert!(u.prec_eq(&v, prec)?);

    assert!(v.q_mul(&v.conjugate()?)?.prec_eq(&j, prec)?);
    assert!(w.set_q_mul(&v.conjugate()?)?.prec_eq(&j, prec)?);
    assert!(v.mul(&v.conjugate()?)?.prec_eq(&j, prec)?);
    assert!(u.mul(&v.conjugate()?)?.prec_eq(&j, prec)?);

    assert!(TryInto::<XMat44F32>::try_into(j.clone())?.prec_eq(&i, prec)?);
    assert!(XMat44F32::try_from(j.clone())?.prec_eq(&i, prec)?);
    assert!(TryInto::<XV4F32>::try_into(i.clone())?.prec_eq(&j, prec)?);
    assert!(XV4F32::try_from(i.clone())?.prec_eq(&j, prec)?);

    let a = XV4F32([1.0, 0.0, 1.0, 1.0]);
    let b = XV4F32([0.0, 1.0, 1.0, 1.0]);
    let c = XV4F32([0.0, -1.0, 1.0, 1.0]);
// println!("{}", v.conjugate()?.q_mul(&a)?); // r2/2, r2/2, 0.0, r2
    assert!(v.conjugate()?.q_mul(&a)?.q_mul(&v)?.prec_eq(&b, prec)?);
// println!("{}", v.conjugate()?.mul(&a)?); // r2/2, r2/2, 0.0, r2
    assert!(v.conjugate()?.mul(&a)?.mul(&v)?.prec_eq(&b, prec)?);
    let m = XMat44F32::try_from(v.clone())?;
    assert!(m.mulvm(&a)?.prec_eq(&b, prec)?);
    assert!(m.mulmv(&a)?.prec_eq(&c, prec)?);

    assert_eq!(rad(0.0f32), 0.0f32);
    assert_eq!(rad(180.0f32), 3.1415927f32);
    assert!(prec_eq(rad(180.0f32), 3.141593f32, prec));
    assert_eq!(rad(360.0f32), 6.2831853f32);
    assert!(prec_eq(rad(360.0f32), 6.283186f32, prec));
    Ok(())
  }

  /// dump bits c_float f32
  /// static_mut_refs to avoid 'ref_mat44_* shared reference to mutable static'
  #[allow(static_mut_refs)]
  #[test]
  fn test_dum() {
    let e = ext::ref_c_u8z(unsafe { &ext::bridge::dump_mat_m44_u8s }, SZBUF);
    let mut u8s = [0u8; SZBUF];
    let l = test_dump(&mut u8s);
    assert_eq!(l, e.len());
    let o = u8s[..l].to_vec();
    assert_eq!(o, e);
//    println!("{}", String::from_utf8(o).expect("utf8"));

    let u = 0xff00ff00ff00ff00u64; // !0: silent 0: constructor/destructor
    assert_eq!(ext::bridge::test_gget(u), u);
    assert_eq!(ext::bridge::Dum::new(u, 123).get(), u);
  }
}
