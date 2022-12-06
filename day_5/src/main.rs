use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() {
    //fuck my life this one is a parsing one fuck me
    let current_path:PathBuf = std::env::current_dir().unwrap();
    let res:PathBuf = rfd::FileDialog::new().set_directory(&current_path).pick_file().unwrap();
    let list:File = File::open(res.as_path()).unwrap();
    let reader:BufReader<File> = BufReader::new(list);
    for buffer in reader.lines() {
        if let Ok(line) = buffer {

        }
    }

}
