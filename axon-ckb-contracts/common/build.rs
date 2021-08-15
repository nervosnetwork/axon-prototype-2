use std::env;
use std::path::Path;

const GCC_FLAG: &str = "gcc_typehash";

/*
capsule is not support to pass feature or cfg while compilation, thus this build.rs lives
 */
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed={}", GCC_FLAG);

    let p = env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR error?");

    let read_file_path = Path::new(&p).join("..")/*.join("target")*/.join("global_config_type_hash.example");

    //let a = read_file_path.as_path().to_str().unwrap().to_string();

    let gcc_flag = env::var(GCC_FLAG).unwrap_or("test_gcc".to_string());

    let gcc_typehash = match &gcc_flag[..] {
        "test_gcc" => "test_gcc",
        "dev_gcc" => "dev_gcc",
        "lina_gcc" => "lina_gcc",
        "aggron_gcc" => "aggron_gcc",
        "custom_gcc" => "custom_gcc",
        _ => panic!("unknown gcc_typehash : {}, this should not happen", gcc_flag),
    };

    if gcc_typehash == "custom_gcc" && !read_file_path.exists() {
        panic!(
            "{}=custom_gcc is set, but file 'global_config_type_hash' does not exist under 'common' package",
            GCC_FLAG
        )
    }

    println!("cargo:rustc-cfg=gcc_typehash=\"{}\"", gcc_typehash);
}
