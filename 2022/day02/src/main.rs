// Advent of Code - Day 2 Rock Paper Scissors
//  Using the strategy guide (data.txt) you need to calculate your score 
//  opponent's keys A, B, C
//  your keys X, Y, Z
//  A, X: Rock = 1
//  B, Y:  Paper = 2
//  C, Z:  Scissors = 3
//  each item played has a value and adds to your score. Bonus points are added depending on the
//  outcome.
//  Win = 6
//  Draw = 3
//  Lose = 0

// parse the data file line by line and split the line at the ' ' to extract the values
// the left column is your opponent's values and the right columns are your values.
//

use std::fs::File;
use std::io::{BufReader, BufRead};




fn main() {
    println!("Advent of Code, Day 2 - Rock, Paper, Scissors");
    let part_1_score = process_part_1_data();
    println!("Part 1 Answer: Your total score is: {}", part_1_score);
    let part_2_score = process_part_2_data();
    println!("Part 2 Answer: Your total score is: {}", part_2_score);

}

fn process_part_1_data() -> i32 {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut score:i32  = 0;

    for line in reader.lines(){
        let str_line = line.unwrap();
        let opp = &str_line[0..1];
        let you = &str_line[2..3];
        score += battle(opp,you);
    }
    score
}

fn process_part_2_data() -> i32 {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut score:i32 = 0;

    for line in reader.lines() {
        let str_line = line.unwrap();
        let opp = &str_line[0..1];
        let you = &str_line[2..3];
        score += next_battle(opp, you);
    }
    println!("Score: {}", score);
    score
}




fn battle(input_1: &str, input_2: &str) -> i32 {
    let result_1: &str;
    let result_2: &str;
    let points: i32;
    match input_1 {
        "A" => result_1 = "Rock",
        "B" => result_1 = "Paper",
        "C" => result_1 = "Scissors",
        _ => result_1 = "invalid", 
    }
   // println!("Oppenent plays: {}", result_1);
    match input_2 {
        "X" => result_2 = "Rock",
        "Y" => result_2 = "Paper",
        "Z" => result_2 = "Scissors",
        _ => result_2 = "invalid",
    }
   // println!("You play: {}", result_2);
   points = get_battle_results(result_1, result_2).0;
   points

}

fn next_battle(input_1: &str, input_2: &str) -> i32 {
    let result_1: &str;
    let result_2: &str;
    let points:i32;
    // X = lose
    // Y = Draw
    // Z = Win
    match input_1 {
        "A" => result_1 = "Rock",
        "B" => result_1 = "Paper",
        "C" => result_1 = "Scissors",
        _ => result_1 = "invalid", 
    }

    // need to determine what result 2 should be assigned to based on result_1 with input_2
    match input_2 {
        // Lose
        "X" => {
            if result_1 == "Rock" {
                result_2 = "Scissors";
            } else if result_1 == "Paper" {
                result_2 = "Rock";
            } else {
                result_2 = "Paper";
            }
        },
        // Draw
        "Y" => result_2 = result_1,
        // Win
        "Z" => {
            if result_1 == "Rock" {
                result_2 = "Paper";
            } else if result_1 == "Paper" {
                result_2 = "Scissors";
            } else {
                result_2 = "Rock";
            }
        },
        _ => result_2 = "invalid",
    } 



    points = get_battle_results(result_1, result_2).0;
    points
}

fn get_battle_results(input_1: &str, input_2: &str) -> (i32, String) {
    let result: &str;
    let mut points: i32 = 0;
    if input_2 == "Rock" {
        if input_1 == "Rock"{
            result = "Draw";
            points = 4;
        } else if input_1 == "Paper" {
            result = "Lose";
            points = 1
        } else {
            result = "Win";
            points = 7;
        }
    } else if input_2 == "Paper" {
        if input_1 == "Rock" {
            result = "Win";
            points = 8;
        } else if input_1 == "Paper" {
            result = "Draw";
            points = 5;
        } else {
            result = "Lose";
            points = 2;
        }
    } else if input_2 == "Scissors" {
        if input_1 == "Rock" {
            result = "Lose";
            points = 3;
        } else if input_1 == "Paper" {
            result = "Win";
            points = 9
        } else {
            result = "Draw";
            points = 6
        }
    } else { println!("Invalid input!");
        result = "invalid";
    }
    return (points, result.to_owned());
}
