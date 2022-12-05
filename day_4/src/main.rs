use std::fs::File;
use std::io::{BufRead, BufReader};

struct Team {
    first: Assignment,
    second: Assignment
}

struct Assignment {
    start: u32,
    end: u32
}

fn parse_assignment(unparsed_assignment: &str) -> Assignment {
    let parts:Vec<&str> = unparsed_assignment.split("-").collect();

    return Assignment {start: parts[0].clone().parse::<u32>().unwrap(), end: parts[1].clone().parse::<u32>().unwrap()}; //ugliest code known to man
}

fn check_contain(check_team: &Team) -> bool {
    return (check_team.first.start <= check_team.second.start && check_team.first.end >= check_team.second.end) || (check_team.second.start <= check_team.first.start && check_team.second.end >= check_team.first.end)
}

fn check_overlap(check_team: &Team) -> bool {
    return (check_team.first.start <= check_team.second.end && check_team.first.start >= check_team.second.start) || (check_team.second.start <= check_team.first.end && check_team.second.start >= check_team.first.start)
}

fn main() {
    let mut contain_counter = 0;
    let mut overlap_counter = 0;
    let current_path = std::env::current_dir().unwrap();
    let res = rfd::FileDialog::new().set_directory(&current_path).pick_file().unwrap();
    let list = File::open(res.as_path()).unwrap();
    let reader = BufReader::new(list);
    for buffer in reader.lines() {
        if let Ok(line) = buffer {
            let assignments = line.split(",").collect::<Vec<&str>>(); //NOTE the ::<... part seems to be generic rust
            //Rust can also get this typing from the var it's going to put the output in
            // let assignments: Vec<&str> = line.split().collect();
            let first_assignment = parse_assignment(assignments[0]);
            let second_assignment = parse_assignment(assignments[1]);
            let temp_team = Team {first: first_assignment, second: second_assignment};
            if check_contain(&temp_team) {
                contain_counter += 1;
            }
            if check_overlap(&temp_team) {
                overlap_counter += 1;
            }
        }
    }
    println!("there should be {} containing teams", contain_counter);
    println!("There should be {} overlapping teams", overlap_counter);
}


