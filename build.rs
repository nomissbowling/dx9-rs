/*
  build.rs for ode-rs

  (with C/C++ Bridge)

  cc-rs bindgen
  and generate link option
*/

extern crate cc;
extern crate bindgen;

use std::path::PathBuf;

macro_rules! pathbufstr {
  ($pb:expr, $s:expr) => { format!("{}", $pb.to_str().expect($s)) };
}

fn main() {
  // to skip build
  // if std::env::var("DOCS_RS").is_ok() { return; }
  // if let Ok(docs_rs) = std::env::var("DOCS_RS") { return; } else { return; }
  let s_path: String = if cfg!(docsrs) {
    // || cfg!(cfg_docs) emmits unexpected_cfgs (rustdoc-args)
    // || cfg!(feature) or || cfg!(test) etc
    // default #[warn(unexpected_cfgs)]
    std::env::var("OUT_DIR").expect("get env OUT_DIR")
  }else{
    ".".to_string()
  }; // to keep lifetime
  let o_path: &str = s_path.as_str();
  let c_opt = if o_path == "." { "-std:c11" }else{ "-std=c++11" };

  // default bindgen should not be inline otherwise link error
  // cc with no default inline when bindgen .generate_inline_functions(true)
  // -fno-implement-inlines : #pragma implementation
  // -fkeep-inline-functions : ***CAUTION***
  // -finline-functions : assume all inline ***CAUTION***
  // -fno-inline : ignore all inline ***CAUTION***
  // -fno-default-inline : not assume inline defined in the class
  // let c_inl = if o_path == "." { "-Ob0" }else{ "-fno-default-inline" };
  // let c_asm = if o_path == "." { "-Fa" }else{ "-S" };

  let mut dxsdk_include = o_path.to_string(); // ".".to_string();
  let mut dxsdk_lib = o_path.to_string(); // ".".to_string();
  if o_path == "." {
    let dxsdk_dir = PathBuf::from(std::env::var("DXSDK_DIR").expect("dxskd"));
    dxsdk_include = pathbufstr!(dxsdk_dir.join("Include"), "Include");
    dxsdk_lib = pathbufstr!(dxsdk_dir.join("Lib").join("x64"), "Lib");
  }

  let mk_cc = |dname: &str, sname: &str, iname: &str, oname: &str| {
    let sd = PathBuf::from(dname);
    let fname = pathbufstr!(sd.join(sname), "invalid path");
    println!("cargo:rerun-if-changed={}", fname);
    cc::Build::new()
      .file(fname)
      .flag(c_opt)
      // .flag("-std=c++11") // gcc
      // .flag("-std:c11") // cl
      // .flag("-std:c++14") // cl
      // .flag(c_inl)
      // .flag(c_asm)
      .include(iname)
      .include(dxsdk_include) // append extra include for only c/cpp source
      .compile(oname)
  };
  if o_path == "." {
    // mk_cc("./src/bridge", "bridge.cpp", "./include", "bridge");
    mk_cc("./ext/bridge", "bridge.cpp", "./include", "bridge");
  }

  let mk_bindings = |hdd: &str, header: &str, rsd: &str, rsfile: &str,
    binl: bool, bcmt: bool| { // inline, comment
    let hd = PathBuf::from(hdd);
    let hf = pathbufstr!(hd.join(header), "invalid path");
    println!("cargo:rerun-if-changed={}", hf);
    let bindings = bindgen::Builder::default()
      .header(hf)
      .generate_inline_functions(binl) // default: false (.hpp inline if true)
      .generate_comments(bcmt) // default: true
      .parse_callbacks(Box::new(bindgen::CargoCallbacks::new())) // 0.72.1
      .generate()
      .expect("Unable to generate bindings");
    let rs = PathBuf::from(rsd);
    bindings
      .write_to_file(rs.join(rsfile))
      .expect("Could not write bindings!");
  };
  if o_path == "." {
    mk_bindings("./include", "bridge.hpp", "./include", "bridge_bindings.rs",
      false, true); // cc should be compiled with option no default inline
  }

  println!("cargo:rustc-link-search={}", dxsdk_lib);
  println!("cargo:rustc-link-lib=d3d9");
  println!("cargo:rustc-link-lib=d3dx9");
}