use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() -> Result<(), Box<dyn std::error::Error>>  {
    println!("Advent of Code Day 1!");

    let path = Path::new("aoc2022_day1_input.txt");
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

    let reindeer = input.split("\n\n");
    let mut reindeer_calories = Vec::new();

    for r in reindeer {
        println!("{:?}", r);
        let food_items = r.split('\n');
        let mut total_calories = 0;
        for item in food_items {
            let calories = match item.parse::<i32>() {
                Err(_) => 0,
                Ok(i) => i
            };
            total_calories += calories;
        }
        println!("total_calories: {:?}", total_calories);
        reindeer_calories.push(total_calories);
    }
    
    reindeer_calories.sort_by(|a, b| b.cmp(a));
    println!("Max calories: {:?}", reindeer_calories[0]);

    println!("Total of top 3: {:?}", reindeer_calories[0] + reindeer_calories[1] + reindeer_calories[2]);
    Ok(())
}