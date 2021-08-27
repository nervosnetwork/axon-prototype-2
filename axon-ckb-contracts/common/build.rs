use std::env;
use std::path::Path;

const GCC_FLAG: &str = "gcc_typehash";
const SUDT_FLAG: &str = "sudt_typehash";

/*
capsule is not support to pass feature or cfg while compilation, thus this build.rs lives
 */
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed={}", GCC_FLAG);
    println!("cargo:rerun-if-env-changed={}", SUDT_FLAG);

    let project_root = env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR error?");

    //================

    let gcc_flag = env::var(GCC_FLAG).unwrap_or("test_gcc".to_string());

    let gcc_typehash = match &gcc_flag[..] {
        "test_gcc" => "test_gcc",
        "dev_gcc" => "dev_gcc",
        "lina_gcc" => "lina_gcc",
        "aggron_gcc" => "aggron_gcc",
        "custom_gcc" => "custom_gcc",
        _ => panic!("unknown gcc_typehash : {}, this should not happen", gcc_flag),
    };

    if gcc_typehash == "custom_gcc" && !Path::new(&project_root).join(".").join("global_config_type_hash").exists() {
        panic!(
            "{}=custom_gcc is set, but file 'global_config_type_hash' does not exist under 'common' package",
            GCC_FLAG
        )
    }

    println!("cargo:rustc-cfg=gcc_typehash=\"{}\"", gcc_typehash);

    //================

    let sudt_flag = env::var(SUDT_FLAG).unwrap_or("test_sudt".to_string());

    let sudt_typehash = match &sudt_flag[..] {
        "test_sudt" => "test_sudt",
        "dev_sudt" => "dev_sudt",
        "lina_sudt" => "lina_sudt",
        "aggron_sudt" => "aggron_sudt",
        "custom_sudt" => "custom_sudt",
        _ => panic!("unknown sudt_typehash : {}, this should not happen", sudt_flag),
    };

    if sudt_typehash == "custom_sudt" && !Path::new(&project_root).join(".").join("sudt_type_hash").exists() {
        panic!(
            "{}=custom_sudt is set, but file 'sudt_type_hash' does not exist under 'common' package",
            SUDT_FLAG
        )
    }

    println!("cargo:rustc-cfg=sudt_typehash=\"{}\"", sudt_typehash);
}
