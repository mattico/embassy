use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let chip_name = env::vars_os()
        .map(|(a, _)| a.to_string_lossy().to_string())
        .find(|x| x.starts_with("CARGO_FEATURE_STM32"))
        .expect("No stm32xx Cargo feature enabled")
        .strip_prefix("CARGO_FEATURE_")
        .unwrap()
        .to_ascii_lowercase();

    let out_dir = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let out_file = out_dir.join("generated.rs").to_string_lossy().to_string();

    let exit_code = Command::new("python")
        .args(&["gen.py", &chip_name, &out_file])
        .status()
        .expect("failed to execute gen.py");

    if !exit_code.success() {
        panic!("gen.py exited with {:?}", exit_code)
    }

    stm32_metapac::peripheral_versions!(
        ($peri:ident, $version:ident) => {
            println!("cargo:rustc-cfg={}", stringify!($peri));
            println!("cargo:rustc-cfg={}_{}", stringify!($peri), stringify!($version));
        };
    );

    let mut chip_and_core = chip_name.split('_');
    let chip = chip_and_core.next().expect("Unexpected stm32xx feature");

    if let Some(core) = chip_and_core.next() {
        println!("cargo:rustc-cfg={}_{}", &chip[..(chip.len() - 2)], core);
    } else {
        println!("cargo:rustc-cfg={}", &chip[..(chip.len() - 2)]);
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=gen.py");
}
