use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("doggo.rs");
    let output = dog_lib::get_message();
    fs::write(
        dest_path,
        format!(
            "\
#[inline]
fn doggo() {{
    unsafe {{
        printf(\"{}\\n\\0\".as_ptr().cast::<i8>());
    }}
}}
",
            output
        ),
    )
    .unwrap();
    println!("cargo:rerun-if-changed=build.rs");
}
