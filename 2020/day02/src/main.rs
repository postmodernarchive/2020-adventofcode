extern crate advent_lib;
use advent_lib::*;

// read file line by line
// each line has criteria if "password" is valid before ("1-3 a:")
// that means, for the password to be valid, the letter `a` neds to be present at least 1 to 3 times

fn main() {
    let lines: Vec<String> = advent_lib::file_to_vec("input.txt".to_string());

    for line in lines {
        println!("criteria {:#?}", get_criteria(line));
    }
}

fn get_criteria(line: String) -> ((u8, u8), char) {
    let criteria: String = line.split(':').collect::<Vec<&str>>()[0].to_string();
    let c: char = criteria.split(' ').collect::<Vec<&str>>()[1].to_string().chars().nth(0).unwrap();

    let nums: Vec<&str> = criteria.split(' ').collect::<Vec<&str>>()[0].split('-').collect::<Vec<&str>>();
    let min: u8 = nums[0].parse::<u8>().unwrap();
    let max: u8 = nums[1].parse::<u8>().unwrap();

    return ((min, max), c);
}
