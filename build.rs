use cmake::Config;
use std::env;
#[cfg(feature = "generate_binding")]
use std::path::PathBuf;

#[cfg(feature = "generate_binding")]
fn generate_bindings() {
    const ALLOW_UNCONVENTIONALS: &'static str = "#![allow(non_upper_case_globals)]\n\
                                                 #![allow(non_camel_case_types)]\n\
                                                 #![allow(non_snake_case)]\n\
                                                 #![allow(improper_ctypes)]\n";

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .raw_line(ALLOW_UNCONVENTIONALS);

    let target = env::var("TARGET").unwrap();
    let target_parts: Vec<&str> = target.split('-').collect();
    let target_os = target_parts.get(2).unwrap_or(&"");
    let target_env = target_parts.get(3).unwrap_or(&"");
    // let target_fn = format!("{}_{}.rs", target_os, target_env);
    let target_fn = if target_env.is_empty() {
        format!("{}.rs", target_os)
    } else {
        format!("{}_{}.rs", target_os, target_env)
    };
    let binding_target_path = PathBuf::new().join("src").join(target_fn);

    // We need to override the target and sysroot for CLang on Windows GNU;
    // see https://github.com/rust-lang/rust-bindgen/issues/1760
    #[cfg(all(windows, target_env = "gnu"))]
    {
        let target = env::var("TARGET").unwrap();

        let bits = if cfg!(target_pointer_width = "32") {
            32
        } else {
            64
        };

        let target_arg = format!("--target={}", target);
        let sysroot_arg = format!(r#"--sysroot=C:\msys64\mingw{}\"#, bits);

        bindings = bindings.clang_args(&[&target_arg, &sysroot_arg]);
    }

    bindings
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(binding_target_path)
        .expect("Could not write binding to the file at `src/lib.rs`");

    println!("cargo:info=Successfully generated binding.");
}

/// Returns an expected default library linking method based on OS or target.
///
/// For Windows, macOS and Linux with musl, it will be true, which denotes `static`.
/// For Linux without musl, it will be false, to denote `dynamic`.
///
/// Note that this is a helper function and may not be called if the `static` feature
/// is enabled or the environment variable `LIBLZHAM_STATIC` or `LZHAM_STATIC` is set.
fn default_library_linking() -> bool {
    #[cfg(any(windows, target_os = "macos", target_env = "musl"))]
    {
        true
    }
    #[cfg(all(unix, target_env = "gnu"))]
    {
        false
    }
}

/// Checks if it is a static build.
///
/// It takes features and environment variables into account first. If none set, it returns
/// answer based on the OS and architecture.
fn is_static_build() -> bool {
    if cfg!(feature = "static") && cfg!(feature = "dynamic") {
        default_library_linking()
    } else if cfg!(feature = "static")
        || env::var("LIBLZHAM_STATIC").is_ok()
        || env::var("LZHAM_STATIC").is_ok()
    {
        println!("cargo:info=Static feature or environment variable found.");

        true
    } else if cfg!(feature = "dynamic") {
        println!("cargo:info=Dynamic feature enabled.");

        false
    } else {
        println!("cargo:info=No feature or environment variable found, linking by default.");

        default_library_linking()
    }
}

fn main() {
    #[cfg(feature = "generate_binding")]
    generate_bindings();

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=stdc++");

    #[cfg(target_os = "windows")]
    #[cfg(target_env = "msvc")]
    println!("cargo:rustc-link-lib=msvcrt");

    #[cfg(target_os = "windows")]
    #[cfg(target_env = "gnu")]
    println!("cargo:rustc-link-lib=stdc++");

    let is_static = is_static_build();

    let dst = if is_static {
        Config::new("lzham_codec")
            .define("CMAKE_BUILD_TYPE", "Release")
            .define("BUILD_SHARED_LIBS", "OFF")
            .build()
    } else {
        Config::new("lzham_codec")
            .define("CMAKE_BUILD_TYPE", "Release")
            .build()
    };

    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    let linking_text = if is_static { "static" } else { "dylib" };

    println!("cargo:rustc-link-lib={}=lzhamdecomp", linking_text);
    println!("cargo:rustc-link-lib={}=lzhamcomp", linking_text);
    println!("cargo:rustc-link-lib={}=lzhamdll", linking_text);
}
