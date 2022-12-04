use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;



fn main() {
    let mut elf_buffer: Vec<String> = Vec::new();
    let mut value: u64 = 0;
    let current_path = std::env::current_dir().unwrap();
    let res = rfd::FileDialog::new().set_directory(&current_path).pick_file().unwrap();
    let list = File::open(res.as_path()).unwrap();
    let reader = BufReader::new(list);
    for buffer in reader.lines() {
        if let Ok(line) = buffer {
            elf_buffer.push(line.clone());
            if (elf_buffer.len() == 3) {
                if let Ok(temp_value) = parse_elfs(elf_buffer[0].clone(), elf_buffer[1].clone(), elf_buffer[2].clone()) {
                    value += temp_value.clone();
                }
                else {
                    println!("Could not find a similarity for lines:\n{}\n{}\n{}\n\n\n", elf_buffer[0], elf_buffer[1], elf_buffer[2])
                }
                elf_buffer = Vec::new()
            }
        }
    }
    println!("{}", value)
}
pub fn parse_elfs(elf_a: String, elf_b: String, elf_c: String) -> Result<u64, String> {
    for char in elf_a.as_bytes() {
        if elf_b.as_bytes().contains(char) && elf_c.as_bytes().contains(char) {
            return Ok(convert_value(char.clone()));
        }
    }
    Err("no similar character found :/".parse().unwrap())
}

fn convert_value(value: u8) -> u64 {
    if (value % 91) < value {
        return u64::from(value.clone()) -96;
    } else {
        return u64::from(value.clone()) -38;
    }
}
