#![feature(env, path, process)]

use std::{env, process};
use std::path::PathBuf;

macro_rules! cmd(
    ($name:expr) => (process::Command::new($name));
);

macro_rules! get(
    ($name:expr) => (env::var($name).unwrap_or(String::new()));
);

macro_rules! set(
    ($name:expr, $value:expr) => (env::set_var($name, $value));
);

macro_rules! run(
    ($command:expr) => (
        assert!($command.stdout(process::Stdio::inherit())
                        .stderr(process::Stdio::inherit())
                        .status().unwrap().success());
    );
);

fn main() {
    let from = PathBuf::new(&get!("CARGO_MANIFEST_DIR")).join("libtar");
    let into = PathBuf::new(&get!("OUT_DIR"));

    set!("CFLAGS", &format!("{} -fPIC", get!("CFLAGS")));

    run!(cmd!(&from.join("configure")).current_dir(&into).arg("--srcdir").arg(&from));
    run!(cmd!("make").current_dir(&into).arg(&format!("-j{}", get!("NUM_JOBS"))));

    println!("cargo:rustc-flags=-L {}", into.join("lib").display());
    println!("cargo:rustc-flags=-l tar:static");
}
