use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};




fn main() {
    let DEBUG = false;
    let current_path = std::env::current_dir().unwrap();
    let res = rfd::FileDialog::new().set_directory(&current_path).pick_file().unwrap();
    let list = File::open(res.as_path()).unwrap();
    let mut elf_counter:u8 = 0;
    let mut calorie_counter:u32 = 0;
    let mut lowest_max_elf: u8 = 0;
    let mut lowest_max_calories: u32 = 0;
    let mut top_elves:Vec<elf> = Vec::new();
    let reader = BufReader::new(list);
    for line in reader.lines(){
        if (DEBUG) {
            println!("DEBUG: parsing line {}", line.as_ref().unwrap())
        }
        if (line.as_ref().unwrap() == "") {
            elf_counter += 1;
            println!("elf {} has {} calories", elf_counter, calorie_counter);
            if (lowest_max_calories < calorie_counter || top_elves.len() < 3) {
                (lowest_max_calories, lowest_max_elf) = check_elf(elf_counter, calorie_counter, lowest_max_elf.clone(), lowest_max_calories.clone(), &mut top_elves);
            }
            calorie_counter = 0;
        } else {
            calorie_counter += line.unwrap().parse::<u32>().unwrap();
        }
    }
    check_elf(elf_counter+1, calorie_counter, lowest_max_elf, lowest_max_calories, &mut top_elves);
    println!("the top 3 elves were:");
    let last = top_elves.pop().unwrap();
    println!("at third place with {} calories we've got elf {}", last.calories, last.id);
    let second = top_elves.pop().unwrap();
    println!("at second place with {} calories we've got elf {}", second.calories, second.id);
    let first = top_elves.pop().unwrap();
    println!("and at first place with {} calories we've got elf {}", first.calories, first.id);
    let combined:u32 = last.calories + second.calories + first.calories;
    println!("which gives us a combined total of {} calories for the top 3", combined)
}

fn check_elf(mut elf_counter: u8, mut calorie_counter: u32, mut lowest_max_elf: u8, mut lowest_max_calories: u32, top_elves: &mut Vec<elf>) -> (u32, u8) {
    top_elves.push(elf { id: elf_counter, calories: calorie_counter });
    top_elves.sort_by(|a, b| b.calories.cmp(&a.calories));
    if (top_elves.len() > 3) {
        top_elves.pop(); //pop last element from the list if there are more than 3 items
    }
    lowest_max_calories = top_elves.last().unwrap().calories.clone();
    lowest_max_elf = top_elves.last().unwrap().id.clone();
    return (lowest_max_calories, lowest_max_elf)
}


struct elf {
    id: u8,
    calories: u32,
}