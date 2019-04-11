use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Put the linker script somewhere the linker can find it.
fn memory_linker_script() {
    let target = env::var("TARGET").unwrap();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let name = env::var("CARGO_PKG_NAME").unwrap();

    fs::copy(
        format!("bin/{}.a", target),
        out_dir.join(format!("lib{}.a", name)),
    ).unwrap();

    println!("cargo:rustc-link-lib=static={}", name);
    println!("cargo:rustc-link-search={}", out_dir.display());

    let dest_path = Path::new(&out_dir);
    let mut f = File::create(&dest_path.join("memory.x"))
        .expect("Could not create file");

    f.write_all(include_bytes!("memory.x"))
        .expect("Could not write file");

    println!("cargo:rustc-link-search={}", dest_path.display());
    println!("cargo:rerun-if-changed=memory.x");
}

fn main() {
    memory_linker_script();
    println!("cargo:rerun-if-changed=build.rs");
}
