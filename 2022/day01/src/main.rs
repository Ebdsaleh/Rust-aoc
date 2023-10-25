// Advent of code 2022 Day 1
// Calorie Counting

use std::fs::File;
use std::io::{BufReader, BufRead};


 
//  std::io::Result<()>

fn main() {
    println!("Advent of Code, Day 1 - Calorie Counting");
    let mut input: Vec<i32>  = process_data();
    let max_calories: i32;
    let second_most_calories: i32;
    let third_most_calories: i32;
    input.sort();
    input.reverse();
    max_calories = input[0];
    second_most_calories = input[1];
    third_most_calories = input[2];
    println!("Max calories: {}", max_calories);
    println!("The answer to Advent of Code Day 1 part 1 is: {}", max_calories);
    let top_three_caloric_sum = max_calories + second_most_calories + third_most_calories;
    println!("The sum of calories carried by the top 3 elves is: {}", top_three_caloric_sum);
}


fn process_data() -> Vec<i32> {

    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new (file);
    let mut input: Vec<i32> = Vec::new();
    let mut _elf_count: i32 = 1;
    let mut calorie_count: i32 = 0;

    for line in reader.lines() {
        let str_line = line.unwrap().to_owned();
        let num: i32;
        
        if str_line.trim() == "" {
            _elf_count += 1;
            input.push(calorie_count);
            calorie_count = 0;
            continue;
        }
        num = str_line.parse::<i32>().unwrap();
        calorie_count += num;
    }
    println!("\nFinished parsing file returning calorie data");
    input
    
}
