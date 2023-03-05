use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

const EXE: &str = "doggosearch";

fn build_tiny_elf() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let asm_path = Path::new(&out_dir).join(format!("{}.asm", EXE));
    let exe_path = Path::new(&out_dir).join(EXE);
    let output = dog_lib::get_message();
    // Tiny ELF
    // https://nathanotterness.com/2021/10/tiny_elf_modernized.html
    fs::write(
        &asm_path,
        format!(
            "\
[bits 64]
file_load_va: equ 4096 * 40
db 0x7f, 'E', 'L', 'F'
db 2
db 1
db 1
db 0
dq 0
dw 2
dw 0x3e
dd 1
dq entry_point + file_load_va
dq program_headers_start
dq section_headers_start
dd 0
dw 64
dw 0x38
dw 1
dw 0x40
dw 3
dw 2
program_headers_start:
dd 1
dd 5
dq 0
dq file_load_va
dq file_load_va
dq string_table
dq string_table
dq 0x200000

section_headers_start:
times 0x40 db 0
dd text_section_name - string_table
dd 1
dq 6
dq file_load_va
dq 0
dq file_end
dd 0
dd 0
dq 16
dq 0

dd string_table_name - string_table
dd 3
dq 0
dq file_load_va + string_table
dq string_table
dq string_table_end - string_table
dd 0
dd 0
dq 1
dq 0

entry_point:
  mov rax, 1
  mov rdi, 1
  mov rsi, file_load_va + message
  mov rdx, message_length
  syscall
  mov rax, 60
  mov rdi, 0
  syscall

message: db `{}\\n`, 0
message_length: equ $ - message

string_table:
db 0
text_section_name:
db `.text`, 0
string_table_name:
db `.shstrtab`, 0
string_table_end:

file_end:
",
            output,
        ),
    )
    .unwrap();

    let output = Command::new("nasm")
        .args(&["-f", "bin", asm_path.as_str(), "-o", exe_path.as_str()])
        .output()
        .expect("failed to assemble");
    if !output.status.success() {
        panic!(
            "failed to assemble: {}",
            String::from_utf8(output.stderr).unwrap(),
        );
    }
    Command::new("chmod")
        .args(&["+x", exe_path.as_str()])
        .output()
        .expect("failed to chmod");
    println!("cargo:rerun-if-changed=build.rs");
}

fn main() {
    build_tiny_elf();
}

trait AsStrExt<'a> {
    fn as_str(&'a self) -> &'a str;
}

impl<'a> AsStrExt<'a> for PathBuf {
    fn as_str(&'a self) -> &'a str {
        self.as_os_str().clone().to_str().unwrap()
    }
}
