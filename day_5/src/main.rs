use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

fn main() {
    //fuck my life this one is a parsing one fuck me
    //let mut count:u32 = 0; //contains the amount of columns
    let mut stack_lines: Vec<String> = Vec::new(); //contains rows of stack data that needs to be parsed
    let mut instruction_lines: Vec<String> = Vec::new(); //contains rows of instruction data that needs to be parsed
    let mut column_numbers: Vec<usize> = Vec::new(); //contains the usize position for each number on the stack. this helps find the characters maybe?
    let possible_numbers: Vec<&str> = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let current_path:PathBuf = std::env::current_dir().unwrap();
    let res:PathBuf = rfd::FileDialog::new().set_directory(&current_path).pick_file().unwrap();
    let list:File = File::open(res.as_path()).unwrap();
    let reader:BufReader<File> = BufReader::new(list);

    for buffer in reader.lines() {
        if let Ok(line) = buffer {
            if column_numbers.len() < 2 {
                let splitted:Vec<&str> = line.split("").collect();
                if splitted.contains(&"1") {
                    let nums:Vec<&str> = line.split(" ").collect();
                    for num in nums {
                        if possible_numbers.contains(&num) {
                            let number = line.find(num).expect("where it go?");
                            column_numbers.push(number);
                        }

                    }
                    continue;
                }
                stack_lines.push(line.clone());
            }
            instruction_lines.push(line.clone());
        }
    }
    println!("there should be {} columns", column_numbers.len());
    let height = stack_lines.len();
    let width = column_numbers.len();
    let mut stacks: Vec<Vec<String>> = Vec::with_capacity(width * height);
    //time to parse the stacks
    let mut line_counter = 0;
    for line in stack_lines {
        let mut col_counter = 0;
        for col in &column_numbers {
            let possible_item:String = line.chars().nth(col.clone()).unwrap().to_string();
            if possible_item != String::from("") {
                stacks[col_counter][line_counter] = possible_item.clone();
            }
            else {
                stacks[col_counter][line_counter] = String::from("0"); //zero if is blank
                println!("huh??? {}", possible_item)
            }
            col_counter += 1;
        }

    }




}
