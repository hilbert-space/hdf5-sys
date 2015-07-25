use std::path::PathBuf;
use std::{env, fs, process};

macro_rules! cmd(
    ($name:expr) => (process::Command::new($name));
);

macro_rules! get(
    ($name:expr) => (env::var($name).unwrap());
);

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
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

    let build = output.join("build");
    let install = output.join("install");

    ok!(fs::create_dir_all(&build));
    ok!(fs::create_dir_all(&install));

    run!(cmd!(source.join("configure")).current_dir(&build)
                                       .arg("--enable-threadsafe")
                                       .arg(&format!("--prefix={}", install.display())));

    run!(cmd!("make").current_dir(&build)
                     .arg(&format!("-j{}", &get!("NUM_JOBS")))
                     .arg("install"));

    println!("cargo:rustc-link-lib=dylib=hdf5");
    println!("cargo:rustc-link-search={}", install.join("lib").display());
}
