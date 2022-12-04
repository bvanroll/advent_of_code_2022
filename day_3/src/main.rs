use std::fs::File;
use std::io::{BufRead, BufReader};
use substring::Substring;

fn main() {
    //A = 65, Z = 90, a = 97, z = 122
    let current_path = std::env::current_dir().unwrap();
    let res = rfd::FileDialog::new().set_directory(&current_path).pick_file().unwrap();
    let list = File::open(res.as_path()).unwrap();
    let reader = BufReader::new(list);
    let mut counter :u64= 0;
    for buffer in reader.lines() {
        if let Ok(line) = buffer {
            let first_half= line.substring(0,line.len()/2).as_bytes();
            let second_half = line.substring(line.len()/2,line.len()).as_bytes();
            for char in first_half {
                if (second_half.contains(&char)) {
                    // println!("{}", (char % 91));
                    // println!("{}", char);
                    if ((char % 91) < *char) {



                        let temp = u64::from(char.clone()) -96;

                        println!("{} is in both parts, LOWERCDASE", temp);
                        counter += temp;
                    }
                    else
                    {
                        let temp = u64::from(char.clone()) -38;
                        println!("{} is in both parts, UPPERCASE", temp);
                        counter += temp;
                    }

                    break;
                }

            }
        }
    }
    println!("complete score is : {}", counter)
}
