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
        if is_password_valid(get_password(line.to_string()), get_criteria(line.to_string())) {
            valid_counter += 1;
        }
    }

    println!("amount of passwords that are valid is: {}", valid_counter);
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

fn is_password_valid(password: String, criteria: Criteria) -> bool {
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
