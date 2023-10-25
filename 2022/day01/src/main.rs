// Advent of code 2022 Day 1
// Calorie Counting

use std::fs::File;
use std::io::{BufReader, BufRead};



//  std::io::Result<()>

fn main() {
    println!("Advent of Code, Day 1 - Calorie Counting");
    let input: Vec<i32>  = process_data();
    let mut max_calories:i32 = 0;
    let mut elf_with_max_calories: usize = 0;

    for (i, v) in input.iter().enumerate() {
        if *v > max_calories {
            max_calories = *v;
            elf_with_max_calories = i;
        } else {
            continue;
        }

    }
    println!("Elf number {} is carrying the most food.", elf_with_max_calories);
    println!("Max calories: {}", max_calories);
    println!("The answer to Adevent of Code Day 1 is: {}", max_calories);
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
