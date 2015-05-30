extern crate pkg_config;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    if env::var("LIBCMARK_SYS_USE_PKG_CONFIG").is_ok() {
        if pkg_config::find_library("libcmark").is_ok() {
            return;
        }
    }

    let src = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());
    let build = dest.join("build");

    let _ = fs::create_dir(&build);

    let mut cmd = Command::new("cmake");
    cmd.arg(&src.join("cmark"))
        .arg("-DBUILD_SHARED_LIBS=OFF")
        .arg("-DBUILD_CLAR=OFF")
        .arg(&format!("-DCMAKE_INSTALL_PREFIX={}", dest.display()))
        .current_dir(&build);
    run(&mut cmd);

    run(Command::new("cmake")
        .arg("--build").arg(".")
        .arg("--target").arg("install")
        .current_dir(&build));

    println!("cargo:root={}", dest.display());

    if env::var("HOST") == env::var("TARGET") {
        prepend("PKG_CONFIG_PATH", dest.join("lib/pkgconfig"));
        if pkg_config::Config::new().statik(true).find("libcmark").is_ok() {
            return;
        }
    }

    println!("cargo:rustc-flags=-l static=cmark");
    println!("cargo:rustc-flags=-L {}", dest.join("lib").display());
}

// From libgit2-sys build script
fn prepend(var: &str, val: PathBuf) {
    let prefix = env::var(var).unwrap_or(String::new());
    let mut path = vec![val];
    path.extend(env::split_paths(&prefix));
    env::set_var(var, &env::join_paths(path).unwrap());
}

fn run(cmd: &mut Command) {
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) => panic!("Failed to execute command: {}", e),
    };

    if !status.success() {
        panic!("Command did not execute successfully: exit {}", status);
    }
}
