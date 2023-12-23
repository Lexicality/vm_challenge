use std::fs::File;
use std::io::Read;

use vm_challenge::machine::VM;

fn main() {
    let data = read_program();
    let mut machine = VM::new(data);
    println!("=== Starting VM ===");
    machine.run();
    println!();
    println!("=== Execution Complete ===");
}

fn read_program() -> Vec<u16> {
    let mut file =
        File::open("challenge.bin").expect("Challenge bin file should be in the working directory");
    let mut raw_data = Vec::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_end(&mut raw_data).unwrap();
    assert_eq!(raw_data.len() % 2, 0);
    raw_data
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()))
        .collect()
}
