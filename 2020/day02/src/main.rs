extern crate advent_lib;

// read file line by line
// each line has criteria if "password" is valid before ("1-3 a:")
// that means, for the password to be valid, the letter `a` neds to be present at least 1 to 3 times

fn main() {
    let lines: Vec<String> = advent_lib::file_to_vec("input.txt".to_string());

    let mut valid_counter: u32 = 0;

    for line in lines {
        //println!("criteria {:#?}", get_criteria(line));
        // using to_string() becuase I do not want to borrow or whatever the term for this is.
        if is_password_valid1(get_password(line.to_string()), get_criteria(line.to_string())) {
            valid_counter += 1;
        }
    }

    println!("amount of passwords that are valid in part one is: {}", valid_counter);

    // just ugly second initialisation of lines
    let lines: Vec<String> = advent_lib::file_to_vec("input.txt".to_string());
    let mut valid_counter: u32 = 0;

    for line in lines {
        //println!("criteria {:#?}", get_criteria(line));
        // using to_string() becuase I do not want to borrow or whatever the term for this is.
        if is_password_valid2(get_password(line.to_string()), get_criteria(line.to_string())) {
            valid_counter += 1;
        }
    }

    println!("amount of passwords that are valid in part one is: {}", valid_counter);
}

struct Criteria {
    min: u8,
    max: u8,
    letter: char,
}

fn get_criteria(line: String) -> Criteria {
    let criteria: String = line.split(':').collect::<Vec<&str>>()[0].to_string();
    let letter: char = criteria.split(' ').collect::<Vec<&str>>()[1].to_string().chars().nth(0).unwrap();

    let nums: Vec<&str> = criteria.split(' ').collect::<Vec<&str>>()[0].split('-').collect::<Vec<&str>>();
    let min: u8 = nums[0].parse::<u8>().unwrap();
    let max: u8 = nums[1].parse::<u8>().unwrap();

    return Criteria {
        min,
        max,
        letter,
    }
}

fn get_password(line: String) -> String {
    line.split(':').collect::<Vec<&str>>()[1].to_string()
}

// part 1
fn is_password_valid1(password: String, criteria: Criteria) -> bool {
    let pass_vec: Vec<char> = password.chars().collect::<Vec<char>>();

    let mut c_counter: u8 = 0;

    for c in pass_vec {
        if c == criteria.letter {
            c_counter += 1;
        }
    }

    if c_counter >= criteria.min && c_counter <= criteria.max {
        return true;
    } else {
        return false;
    }
}

// part 2
fn is_password_valid2(password: String, criteria: Criteria) -> bool {
    let pass_vec: Vec<char> = password.chars().collect::<Vec<char>>();

    let min: usize = criteria.min as usize;
    let max: usize = criteria.max as usize;

    if pass_vec[min] == criteria.letter &&
    pass_vec[max] == criteria.letter {

            return false;
    } else if pass_vec[min] != criteria.letter &&
    pass_vec[max] != criteria.letter {
        return false;
    } else if pass_vec[min] == criteria.letter ||
    pass_vec[max] == criteria.letter {
        return true;
    } else {
        // not even sure, if it is ever able to reach this case
        println!("the \"unreachable\" case has been reached !!");
        return false;
    }
}
