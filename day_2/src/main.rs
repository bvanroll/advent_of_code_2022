use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use phf::phf_map;


static scores: phf::Map<&'static str, u8> = phf_map! (
    "A" => 1, //rock
    "B" => 2, //paper
    "C" => 3, //scizzors
);

// figuring out a way to do this :/
// 1 2 = lose
// 1 3 = win
// 2 1 = win
// 2 3 = lose
// 3 1 = lose
// 3 2 = win

static possibilities: phf::Map<&'static u8, &'static u8> = phf_map! {
    12 => 0,
    13 => 6,
    21 => 6,
    23 => 0,
    31 => 0,
    32 => 6,
    11 => 3,
    22 => 3,
    33 => 3,
};

static shape_map: phf::Map<&'static String, &'static String> = phf_map! {
    "X" => "A",
    "Y" => "B",
    "Z" => "C",
};

struct Play {
    them: u8,
    you: u8
}


fn main() {
    println!("Hello, world!");

    let current_path = std::env::current_dir().unwrap();
    let res = rfd::FileDialog::new().set_directory(&current_path).pick_file().unwrap();
    let book = File::open(res.as_path()).unwrap();

    let reader = BufReader::new(book);
    let mut complete_score:u32 = 0;
    for line in reader.lines() {
        let play = parse_line(line.unwrap());
        let score = get_score(play);
        println!("score for line {} is {}", line.unwrap(), score);
        complete_score += score;
    }

    println!("the completed score for the whole book is {}",complete_score)
}

fn parse_line(line: String) -> Play {
    let mut temp = line.split(' ').collect::<Vec<&str>>();
    return Play {them: scores[temp[0].parse().unwrap()], you: scores[temp[1].parse().unwrap()] }
}

fn get_score(p: Play) -> u8 {
    return possibilities[p.you*10+p.them]+scores[p.you];
}
