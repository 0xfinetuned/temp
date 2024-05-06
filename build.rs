use hex;
use std::fs;

fn main() {

    let methods = risc0_build::embed_methods();
    for method in methods {
        let _ = fs::write("target/program_elf", method.elf);
    }

}
