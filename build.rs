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
        .header("ext/pal/pal.h")
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
    
    // The "static" feature means that we compile the PAL source directly and
    // link it. If we're not using that feature, then we need to find the
    // library and link that instead.
    #[cfg(not(feature = "static"))]
    {
        // See if PAL_LIB is defined. If so, use it to search and link the library.
        match std::env::var("PAL_LIB") {
            Ok(lib) => {
                println!("cargo:rustc-link-search=native={}", lib);
                println!("cargo:rustc-link-lib=pal");
            }

            // Search via pkg-config.
            Err(_) => {
                pkg_config::probe_library("pal")
                .unwrap_or_else(|_| panic!("Couldn't find the PAL library via pkg-config. Please set the PAL_LIB environment variable to the location of `libpal.so`"));
            }
        }

        // Because pkg-config-rs is very restrictive of allowing things to be
        // compiled statically, manually specify that we should link statically here
        // (https://github.com/rust-lang/pkg-config-rs/issues/102).
        if infer_static("PAL") {
            println!("cargo:rustc-link-lib=static=pal");
        }
    }

    #[cfg(feature = "static")]
    {
        // Change this directory if the source code is updated.
        let pal_project_dir = std::path::PathBuf::from("ext/pal");
        if !pal_project_dir.exists() {
            panic!(
                "Expected to find PAL source directory {}",
                pal_project_dir.display()
            );
        }

        // Translate rustc optimisation levels to things a C compiler can
        // understand. I don't know if all C compilers agree here, but it should
        // at least work for gcc.
        let opt_level: String = match std::env::var("OPT_LEVEL").as_ref().map(|o| o.as_str()) {
            Err(_) => panic!("Something wrong with OPT_LEVEL"),
            // gcc doesn't handle 'z'. Just set it to 's', which also optimises
            // for size.
            Ok("z") => "s",
            Ok(o) => o,
        }
        .to_string();
        let dst = autotools::Config::new(pal_project_dir)
            .disable_shared()
            .cflag("-Wall")
            .cflag(format!("-O{}", opt_level))
            .cflag("-fPIE")
            .build();

        println!("cargo:rustc-link-search=native={}/lib", dst.display());
        println!("cargo:rustc-link-lib=static=pal");
    }
}
