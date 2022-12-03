--- Day 2: Rock Paper Scissors ---

The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to Play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should Play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:
```
A Y
B X
C Z
```
This strategy guide predicts and recommends the following:

    In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
    In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
    The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.

In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?

To begin, get your puzzle input.




## Notes

I know there are more optimal solutions to this (like using char to int calculations etc), but i decided i wanted to use a map for this one.
At some point i wanted to define the map as a global variable and stumbled upon [a stackoverflow post](https://stackoverflow.com/questions/60273064/rust-best-practices-when-specifying-a-constant-hash-map) which had this solution without using any crates:
```rust
use std::collections::HashMap;

pub const COUNTRIES: HashMap<&str, &str> = [
    ("UK", "United Kingdom"),
    ("US", "United States")
].iter().cloned().collect();
```

I really didn't like how this looked in the end so i decided to go for the suggested answer and used a crate called phf.

```rust
use phf::{phf_map};

static COUNTRIES: phf::Map<&'static str, &'static str> = phf_map! {
    "US" => "United States",
    "UK" => "United Kingdom",
};
```

this gave some issues when trying to import the crate (something with the macro's).
I couldn't find the fix for this in the documentation so i decided in the end to go back to the ugly but native solution

example of the errors:

```
error[E0432]: unresolved import `phf::phf_map`
 --> src\main.rs:5:5
  |
5 | use phf::phf_map;
  |     ^^^^^^^^^^^^ no `phf_map` in the root
```


```
error: cannot determine resolution for the macro `phf_map`
 --> src\main.rs:7:49
  |
7 | pub static scores: phf::Map<&'static str, u8> = phf_map! (
  |                                                 ^^^^^^^
  |
  = note: import resolution is stuck, try simplifying macro imports
```

--update
all of that did not work... going to try to figure something out. i really didn't wanna initialise the maps on start.

--update 2
we're skipping global, first answer [here](https://stackoverflow.com/a/27826181) says avoid global as much as possible followed by 19 paragraphs of explanation so i'm guessing this guy knows his stuff. Gonna initialise some map and pass it's ref to the functions needed
