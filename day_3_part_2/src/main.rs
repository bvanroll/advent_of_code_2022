use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;



fn main() {
    let mut elfBuffer: Vec<String> = Vec::new();
    let mut value: u64 = 0;
    let current_path = std::env::current_dir().unwrap();
    let res = rfd::FileDialog::new().set_directory(&current_path).pick_file().unwrap();
    let list = File::open(res.as_path()).unwrap();
    let reader = BufReader::new(list);
    for buffer in reader.lines() {
        if let Ok(line) = buffer {
            elfBuffer.push(line.clone());
            if (elfBuffer.len() == 3) {
                value += parse_elfs(elfBuffer[0].clone(), elfBuffer[1].clone(), elfBuffer[2].clone());
                elfBuffer = Vec::new()
            }
        }
    }
    println!("{}", value)
}
pub fn parse_elfs(elf_a: String, elf_b: String, elf_c: String) -> u64{
    for char in elf_a.as_bytes() {
        if elf_b.as_bytes().contains(char) && elf_c.as_bytes().contains(char) {
            return convert_value(char.clone());
        }
    }
    return 0;//nothing found
    //TODO when nothing found make this throw an err and catch the err above
}

fn convert_value(value: u8) -> u64 {
    if (value % 91) < value {
        return u64::from(value.clone()) -96;
    } else {
        return u64::from(value.clone()) -38;
    }
}
