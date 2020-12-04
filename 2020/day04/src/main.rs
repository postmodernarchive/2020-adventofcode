extern crate advent_lib;

// passports:
// key:value pairs
// pairs are separated by spaces or newlines
// passports are separated by empty newlines
// passport contains fields:
//    byr (Birth Year)
//    iyr (Issue Year)
//    eyr (Expiration Year)
//    hgt (Height)
//    hcl (Hair Color)
//    ecl (Eye Color)
//    pid (Passport ID)
//    cid (Country ID)
// all fields are reqiured except cid -> country id, which is optional
// if any other field than cid is missing -> passport is invalid


fn main() {
    let lines: Vec<String> = advent_lib::file_to_vec("input.txt".to_string());

    //separate lines into vector, separated by empty newlines ("\n")
    let mut passports: Vec<String> = vec![];
    let mut passport: String = "".to_string();

    for line in lines {
        if line.is_empty() || line == "\n".to_string() {
            passports.push(passport.to_owned());
            let passport = String::new();
        } else {
            passport += &line;
        }
    }

    println!("{:#?}", passports);
}

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    pub fn Parse(str: String) -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,

        }
    }

    pub fn is_valid(&self) -> bool {
        false
    }
}
