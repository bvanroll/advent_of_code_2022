use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;


// pub const scores: HashMap<&str, u8> = HashMap::from([
//     ("A",1), //rock
//     ("B",2), //paper
//     ("C",3) //scizzors
// ]);

// figuring out a way to do this :/
// 1 2 = lose
// 1 3 = win
// 2 1 = win
// 2 3 = lose
// 3 1 = lose
// 3 2 = win
//
// static possibilities: HashMap<i32,i32> = HashMap::from([
//     (12,0),
//     (13,6),
//     (21,6),
//     (23,0),
//     (31,0),
//     (32,6),
//     (11,3),
//     (22,3),
//     (33,3)
// ]);

// static shape_map: HashMap<&str, &str> = [
//     ("X","A"),
//     ("Y","B"),
//     ("Z","C")
// ].iter().cloned().collect();

struct Play {
    them: i32,
    you: i32
}


fn main() {
    println!("Hello, world!");
    let scores: HashMap<&str, i32> = HashMap::from([
        ("A",1), //rock
        ("B",2), //paper
        ("C",3), //scizzors
        ("X",1),
        ("Y",2),
        ("Z",3)

    ]);

    let possibilities: HashMap<i32,i32> = HashMap::from([
        (12,0),
        (13,6),
        (21,6),
        (23,0),
        (31,0),
        (32,6),
        (11,3),
        (22,3),
        (33,3)
    ]);



    let current_path = std::env::current_dir().unwrap();
    let res = rfd::FileDialog::new().set_directory(&current_path).pick_file().unwrap();
    let book = File::open(res.as_path()).unwrap();

    let reader = BufReader::new(book);
    let mut complete_score:i32 = 0;
    for buffer in reader.lines() {
        if let Ok(unparsed_play) = buffer { //found the way to handle these results :)
            let play = parse_line(&unparsed_play, &scores);
            let score = get_score(play, &possibilities);
            println!("score for line {} is {}", &unparsed_play, score);
            complete_score += score;
        }
    }

    println!("the completed score for the whole book is {}",complete_score)
}

fn parse_line(line: &String, scores: &HashMap<&str, i32>) -> Play {
    let mut temp = line.split(' ').collect::<Vec<&str>>();
    return Play {them: scores[temp[0]].clone(), you: scores[temp[1]].clone() }
}

fn get_score(p: Play, possibilities: &HashMap<i32, i32>) -> i32 {
    return possibilities[&(p.you*10+p.them)]+&p.you;
}
