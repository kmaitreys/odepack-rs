// build.rs
extern crate cc;
use std::env;

fn main() {
    let target = env::var("TARGET").unwrap();

    // Try to compile with appropriate settings for the target
    let mut build = cc::Build::new();

    build.file("src/blkdta000.f"); // dependency routines
    build.file("src/opkda1.f");
    build.file("src/opkda2.f");
    build.file("src/dlsodes.f"); // main solver
    // build.file("src/odepack.f"); // remaining ODEPACK routines

    // Configure for different targets
    if target.contains("apple") {
        // macOS/iOS specific configuration
        build.compiler("gfortran");
        build.flag("-w");
        build.flag("-O3");
        build.flag("-std=legacy");
        build.flag("-fPIC");
        build.flag("-mcpu=apple-m3");
        build.flag("-mtune=native");

        // Check if gfortran is available
        if std::process::Command::new("gfortran")
            .arg("--version")
            .output()
            .is_err()
        {
            panic!("gfortran not found. Install with: brew install gcc");
        }
    } else {
        // Linux and other Unix-like systems
        build.compiler("gfortran");
        build.flag("-w");
        build.flag("-O3");
        build.flag("-fPIC");
        build.flag("-std=legacy");
    }

    build.compile("libdlsodes.a");

    println!("cargo:rustc-link-lib=static=dlsodes");

    // Platform-specific library linking
    if target.contains("apple") {
        // Find gfortran library path on macOS
        let gfortran_lib_path = find_gfortran_lib_path();
        if let Some(path) = gfortran_lib_path {
            println!("cargo:rustc-link-search=native={}", path);
        }
        println!("cargo:rustc-link-lib=dylib=gfortran");

        // Also link other required Fortran runtime libraries
        println!("cargo:rustc-link-lib=dylib=gcc_s.1");
        println!("cargo:rustc-link-lib=dylib=quadmath");
    } else {
        println!("cargo:rustc-link-lib=dylib=gfortran");
    }

    println!("cargo:rerun-if-changed=src/blkdta000.f");
    println!("cargo:rerun-if-changed=src/opkda1.f");
    println!("cargo:rerun-if-changed=src/opkda2.f");
    println!("cargo:rerun-if-changed=src/dlsodes.f");
}

fn find_gfortran_lib_path() -> Option<String> {
    use std::process::Command;

    // Try to find gfortran library path using gfortran itself
    if let Ok(output) = Command::new("gfortran")
        .args(["-print-file-name=libgfortran.dylib"])
        .output()
        && output.status.success()
    {
        let path_full = String::from_utf8_lossy(&output.stdout);
        let path_str = path_full.trim();
        if let Some(parent) = std::path::Path::new(path_str).parent() {
            return Some(parent.to_string_lossy().to_string());
        }
    }

    // Fallback: common Homebrew paths
    let common_paths = [
        "/opt/homebrew/lib/gcc/13",
        "/opt/homebrew/lib/gcc/12",
        "/opt/homebrew/lib/gcc/11",
        "/usr/local/lib/gcc/13",
        "/usr/local/lib/gcc/12",
        "/usr/local/lib/gcc/11",
    ];

    for path in &common_paths {
        if std::path::Path::new(path).exists() {
            return Some(path.to_string());
        }
    }

    None
}
