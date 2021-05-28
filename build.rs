use std::{env, path::PathBuf};

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Most of this file is adapted from the pal-sys crate
// (https://github.com/cjordan/rust-pal/blob/master/pal-sys/build.rs)

// This code is adapted from pkg-config-rs
// (https://github.com/rust-lang/pkg-config-rs).
#[cfg(not(feature = "static"))]
#[allow(clippy::if_same_then_else, clippy::needless_bool)]
fn infer_static(name: &str) -> bool {
    if std::env::var(format!("{}_STATIC", name.to_uppercase())).is_ok() {
        true
    } else if std::env::var(format!("{}_DYNAMIC", name.to_uppercase())).is_ok() {
        false
    } else if std::env::var("PKG_CONFIG_ALL_STATIC").is_ok() {
        true
    } else if std::env::var("PKG_CONFIG_ALL_DYNAMIC").is_ok() {
        false
    } else {
        false
    }
}

fn main() {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("include/wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // include all functions starting with pal
        .allowlist_function("pal.*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Allow user to specify where PAL is installed
    let pal_lib = std::env::var("PAL_LIB").unwrap_or(String::from("/usr/local/lib"));
    println!("cargo:rustc-link-search=native={}", pal_lib);

    // The "static" feature means that we compile the PAL source directly and
    // link it. If we're not using that feature, then we need to find the
    // library and link that instead.
    #[cfg(not(feature = "static"))]
    {
        println!("cargo:rustc-link-lib=pal");

        // Because pkg-config-rs is very restrictive of allowing things to be
        // compiled statically, manually specify that we should link statically here
        // (https://github.com/rust-lang/pkg-config-rs/issues/102).
        if infer_static("PAL") {
            println!("cargo:rustc-link-lib=static=pal");
        }
    }

    #[cfg(feature = "static")]
    {
        // println!("cargo:warning=pal is statically linked");
        println!("cargo:rustc-link-lib=static=pal");
    }

    let erfa_lib = std::env::var("ERFA_LIB").unwrap_or(String::from("/usr/lib/x86_64-linux-gnu"));

    println!("cargo:rustc-link-search=native={}", erfa_lib);
    println!("cargo:rustc-link-lib=erfa");
}
