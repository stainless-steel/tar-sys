#![feature(io, path)]

use std::old_io as io;
use std::os;

macro_rules! cmd(
    ($name:expr) => (io::process::Command::new($name));
);

macro_rules! fmt(
    ($($arg:tt)*) => (&format!($($arg)*)[]);
);

macro_rules! get(
    ($name:expr) => (os::getenv($name).unwrap_or("".to_string()));
);

macro_rules! set(
    ($name:expr, $value:expr) => (os::setenv($name, $value));
);

macro_rules! run(
    ($command:expr) => (
        assert!($command.stdout(io::process::InheritFd(1))
                        .stderr(io::process::InheritFd(2))
                        .status().unwrap().success());
    );
);

fn main() {
    let from = Path::new(get!("CARGO_MANIFEST_DIR")).join("libtar");
    let into = Path::new(get!("OUT_DIR"));

    set!("CFLAGS", fmt!("{} -fPIC", get!("CFLAGS")));

    run!(cmd!(from.join("configure")).cwd(&into).arg("--srcdir").arg(&from));
    run!(cmd!("make").cwd(&into).arg(fmt!("-j{}", get!("NUM_JOBS"))));

    println!("cargo:rustc-flags=-L {}", into.join("lib").display());
    println!("cargo:rustc-flags=-l tar:static");
}
