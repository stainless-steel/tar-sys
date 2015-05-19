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
    let source = PathBuf::from(&get!("CARGO_MANIFEST_DIR")).join("source");
    let output = PathBuf::from(&get!("OUT_DIR"));

    set!("CFLAGS", &format!("{} -fPIC", get!("CFLAGS")));

    run!(cmd!(&source.join("configure")).current_dir(&output).arg("--srcdir").arg(&source));
    run!(cmd!("make").current_dir(&output).arg(&format!("-j{}", get!("NUM_JOBS"))));

    println!("cargo:rustc-link-search={}", output.join("lib").display());
    println!("cargo:rustc-link-lib=static=tar");
}
