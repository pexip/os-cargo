use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=LIBZ_SYS_STATIC");
    println!("cargo:rerun-if-changed=build.rs");
    let host = env::var("HOST").unwrap();
    let target = env::var("TARGET").unwrap();

    let host_and_target_contain = |s| host.contains(s) && target.contains(s);

    // never build static
    let want_static = false;
    if !want_static &&
       !target.contains("msvc") && // pkg-config just never works here
       !(host_and_target_contain("apple") ||
         host_and_target_contain("freebsd") ||
         host_and_target_contain("dragonfly"))
    {
        // Don't print system lib dirs to cargo since this interferes with other
        // packages adding non-system search paths to link against libraries
        // that are also found in a system-wide lib dir.
        let zlib = pkg_config::Config::new()
            .cargo_metadata(true)
            .print_system_libs(false)
            .probe("zlib");
        match zlib {
            Ok(_) => return,
            Err(e) => {
                println!("cargo-warning={}", e.to_string())
            }
        }
    }

    // All android compilers should come with libz by default, so let's just use
    // the one already there. Likewise, Haiku always ships with libz, so we can
    // link to it even when cross-compiling.
    if target.contains("android") || target.contains("haiku") {
        println!("cargo:rustc-link-lib=z");
        return;
    }

    panic!("librust-libz-sys only supports linking with system lib on Debian!");
}
