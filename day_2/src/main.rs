use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() -> Result<(), Box<dyn std::error::Error>>  {
    println!("Advent of Code Day 2!");

    let path = Path::new("input.txt");
    let display = path.display();

    let mut input_file = match File::open(&path) {
        Err(why) => panic!("couldn't open file {}: {}", display, why),
        Ok(file) => file
    };

    let mut input = String::new();
    match input_file.read_to_string(&mut input) {
        Err(why) => panic!("couldn't read file to string {}", why),
        Ok(_) => ()
    };

    let strategies = input.split('\n');
    let mut total_score_p1 = 0;
    let mut total_score_p2 = 0;

    for strat in strategies {
        let mut strat_score = 0;
        let moves: Vec<Option<char>> = strat.split(" ").map(|s| s.chars().next()).collect();

        let move_a = match moves[0] {
            Some(m) => m,
            None => continue
        };

        let move_b = match moves[1] {
            Some(m) => m,
            None => continue
        };

        // Part 1
        strat_score += match move_b {
            'X' => 1, // rock
            'Y' => 2, // paper
            'Z' => 3, // scissors
            x => panic!("Unexpected value {:?}!", x)
        };

        strat_score += match move_a {
            'A' => {
                match move_b {
                    'X' => 3, // tie
                    'Y' => 6, // win
                    'Z' => 0, // lose
                    x => panic!("Unexpected value {:?}!", x)
                }
            },
            'B' => {
                match move_b {
                    'X' => 0, // lose
                    'Y' => 3, // tie
                    'Z' => 6, // win
                    x => panic!("Unexpected value {:?}!", x)
                }
            },
            'C' => {
                match move_b {
                    'X' => 6, // win
                    'Y' => 0, // lose
                    'Z' => 3, // tie
                    x => panic!("Unexpected value {:?}!", x)
                }
            },
            x => panic!("Unexpected value {:?}!", x)
        };

        total_score_p1 += strat_score;

        // Part 2
        strat_score = 0;

        match move_b {
            'X' => match move_a {
                // Need to lose!
                'A' => strat_score += 0 + 3, // scissors
                'B' => strat_score += 0 + 1, // rock
                'C' => strat_score += 0 + 2, // paper
                x => panic!("Unexpected value {:?}!", x)
            },
            'Y' => match move_a {
                // need to draw!
                'A' => strat_score += 3 + 1, // rock
                'B' => strat_score += 3 + 2, // paper
                'C' => strat_score += 3 + 3, // scissors
                x => panic!("Unexpected value {:?}!", x)
            },
            'Z' => match move_a {
                // need to win!
                'A' => strat_score += 6 + 2, // paper
                'B' => strat_score += 6 + 3, // scissors
                'C' => strat_score += 6 + 1, // rock
                x => panic!("Unexpected value {:?}!", x)
            },
            x => panic!("Unexpected value {:?}!", x)
        };

        total_score_p2 += strat_score;
    }

    println!("Part 1 Result: {:?}", total_score_p1);
    println!("Part 2 Result: {:?}", total_score_p2);

    Ok(())
}
