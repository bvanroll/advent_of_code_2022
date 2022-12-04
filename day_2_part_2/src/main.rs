use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// struct play {
//     them: str,
//     you: str
// }

fn main() {

    let current_path = std::env::current_dir().unwrap();
    let res = rfd::FileDialog::new().set_directory(&current_path).pick_file().unwrap();
    let book = File::open(res.as_path()).unwrap();


    let reader = BufReader::new(book);
    let mut complete_score:i32 = 0;

    let scores: HashMap<&str, i32> = HashMap::from([
        ("A", 1), //rock
        ("B", 2), //paper
        ("C", 3) //scizzors
    ]);

    let lose_options: HashMap<i32,i32> = HashMap::from([
        (1,3),//scissors lose to rock
        (2,1),//rock lose to paper
        (3,2)//paper lose to scissors
    ]);

    let win_options: HashMap<i32,i32> = HashMap::from([
        (1,2),//paper beats rock
        (2,3),//scissors beat paper
        (3,1)//rock beats scissors
    ]);


    for buffer in reader.lines() {
        if let Ok(line) = buffer {
            let unparsed_play = line.split(' ').collect::<Vec<&str>>();
            complete_score += calculate_score(&unparsed_play, &scores, &lose_options, &win_options);
        }
    }

    println!("total score = {}", complete_score)




}

fn calculate_score(current_play: &Vec<&str>, scores: &HashMap<&str, i32>, lose_options: &HashMap<i32, i32>, win_options: &HashMap<i32, i32>) -> i32 {
    let them:i32 = scores[current_play[0]];
    let mut score = 0;
    if (current_play[1] == String::from("X")) {
        //lose
        let mut score = (0 + lose_options[&them]);
        println!("current play:\nthem: {}, \nus: {}\nscore: {}\nformula: gamestate {} + option {} = score {}", current_play[0],current_play[1], score, 0, lose_options[&them], score);
        return score
    } else if (current_play[1] == String::from("Y")) {
        //draw
        let mut score = 3 + them;
        println!("current play:\nthem: {}, \nus: {}\nscore: {}\nformula: gamestate {} + option {} = score {}", current_play[0],current_play[1], score, 3, them, score);
        return score;
    } else {
        //win
        let mut score = 6 + win_options[&them];
        println!("current play:\nthem: {}, \nus: {}\nscore: {}\nformula: gamestate {} + option {} = score {}", current_play[0],current_play[1], score,6,win_options[&them], score);
        return score;
    }
}


