// Advent of Code Day 3 - Rucksack Reorganisation.
// Instructions find the common item in each ruscksack.
// Each rucksack has 2 compartments and is completely filled.
// Determine the compartments by the length of each string, the first compartment will end and the
// second compartment will start in the middle.
// Each item is represented by a character, and has a value lowercase characters have a priority value from
// 1 to 26, Uppercase characters have a priority value from 27 to 52.
// Once you have determined the common item in each compartment, store the priority value and move
// on to the next rucksack, 
// PART ONE:
// sum the priority values together once all rucksacks have been examined
// and provide the answer
//
// use the sample data provided to get your code working.
// PART TWO:
// The elves are assigned into groups of 3, examine the whole contents of each rucksack and compare
// the items to find the priority item that is assigned to the group. 
// Sum the that group's priority item value with the following group. 
// One priority item value per group.


/* part two group example:
//
// group 1, 3 rucksacks group prority item = 'r'
   vJrwpWtwJgWrhcsFMMfFFhFp
   jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
   PmmdzqPrVvPwwTWBwg


// group 2, 3 rucksacks, group priority item = 'Z'
   wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
   ttgJtRGJQctTZtZT
   CrZsJsPPZsGzwwsLwLmpwMDw
*/

// sample data
/*
   vJrwpWtwJgWrhcsFMMfFFhFp
   jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
   PmmdzqPrVvPwwTWBwg
   wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
   ttgJtRGJQctTZtZT
   CrZsJsPPZsGzwwsLwLmpwMDw
   */

use std::{io::{BufReader, BufRead}, fs::File};


fn main() {
    println!("Advent of Code Day 3 - Rucksack Reorganisation.");
    
    let mut sum: i32 = process_part_1_data();
    
    println!("The Sum of the common items is : {}", sum);
    sum = process_part_2_data();
    println!("The Sum of the group items is: {}", sum);
}

fn find_common_item(comp_1: &str, comp_2: &str) -> String {
    let mut found_item: char = ' ';
    let common_item: String;
    for item in comp_1.chars() {
        if comp_2.contains(item){
           // println!("common item is: {}", item);
            found_item = item;
            break;
        }
    }
    common_item = found_item.to_string();
    common_item
}

fn get_item_priority(item: &str) -> i32 {
    let mut priority: i32 = 0;
    match item {
        "a" => priority = 1,
        "b" => priority = 2,
        "c" => priority = 3,
        "d" => priority = 4,
        "e" => priority = 5,
        "f" => priority = 6,
        "g" => priority = 7,
        "h" => priority = 8,
        "i" => priority = 9,
        "j" => priority = 10,
        "k" => priority = 11,
        "l" => priority = 12,
        "m" => priority = 13,
        "n" => priority = 14,
        "o" => priority = 15,
        "p" => priority = 16,
        "q" => priority = 17,
        "r" => priority = 18,
        "s" => priority = 19,
        "t" => priority = 20,
        "u" => priority = 21,
        "v" => priority = 22,
        "w" => priority = 23,
        "x" => priority = 24,
        "y" => priority = 25,
        "z" => priority = 26,
        "A" => priority = 27,
        "B" => priority = 28,
        "C" => priority = 29,
        "D" => priority = 30,
        "E" => priority = 31,
        "F" => priority = 32,
        "G" => priority = 33,
        "H" => priority = 34,
        "I" => priority = 35,
        "J" => priority = 36,
        "K" => priority = 37,
        "L" => priority = 38,
        "M" => priority = 39,
        "N" => priority = 40,
        "O" => priority = 41,
        "P" => priority = 42,
        "Q" => priority = 43,
        "R" => priority = 44,
        "S" => priority = 45,
        "T" => priority = 46,
        "U" => priority = 47,
        "V" => priority = 48,
        "W" => priority = 49,
        "X" => priority = 50,
        "Y" => priority = 51,
        "Z" => priority = 52,

        _=> println!("invalid item."),
    }
    priority
}

fn process_part_1_data() -> i32 {
    let mut sum = 0;
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines(){
        let str_line = line.unwrap();
        let count = str_line.len();
        //println!("{} characters long in this line.", count);
        let split = count / 2;
        let comp_1 = &str_line[0..split];
        let comp_2 = &str_line[split..];
        //println!("comp1: {}\ncomp2: {}", comp_1, comp_2);
        let common_item = find_common_item(comp_1, comp_2);
        let priority = get_item_priority(&common_item);
        sum += priority;

    }
    sum
}

fn process_part_2_data() -> i32 {
    let mut sum: i32;
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);
    let mut count = 0;
    let mut group: Vec<String> = Vec::new();
    let mut rucksacks: Vec<String> = Vec::new();

    sum = 0;
    // store all lines in the file into a vector 
    // to allow the data to be compared.
    for line in reader.lines(){
        rucksacks.push(line.unwrap().to_owned());
    }
    for sack in rucksacks {
        count += 1;
        group.push(sack);       
        if count == 3 {
            count = 0;
            sum += get_item_priority(&find_group_item(&group));
            group.clear();
            
        } else {
            continue;
        }
    }



    sum


}

fn find_group_item(group: &Vec<String>) -> String{
    let group_item: String;
    let mut found_item: char = ' ';
    let mut sack_1: &str = " ";
    let mut sack_2: &str = " ";
    let mut sack_3: &str = " ";
    let mut count = 0;
    for rucksack in group {
        count += 1;
        if count == 1 {
            sack_1 = &rucksack;
        }
        else if count == 2 {
            sack_2 = &rucksack;
        } else {
            sack_3 = &rucksack;
        }
    }
    for item in sack_1.chars() {
        if sack_2.contains(item) && sack_3.contains(item) {
            found_item = item; 
            break;
        }

    }
    group_item = found_item.to_string();
    group_item

}
