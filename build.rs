extern crate nasm_rs;

fn main() {
    nasm_rs::compile_library_args("libloader.a", &["src/loader/boot.s"], &["-felf32"]);
}

