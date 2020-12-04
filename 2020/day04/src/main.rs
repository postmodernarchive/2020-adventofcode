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
    let lines: Vec<String> = advent_lib::file_to_vec("input-test.txt".to_string());

    //separate lines into vector, separated by empty newlines ("\n")
    let mut passport_str: Vec<String> = vec![];
    let mut passport: String = "".to_string();

    for line in lines.iter() {

        if line.is_empty() || *line == "\n".to_string() {
            passport_str.push(passport.to_owned());
            passport = String::new();
        } else {
            passport += &line;
        }

        if line == lines.last().expect("there arent any lines or something i think") {
            passport_str.push(passport.to_owned());
        }
    }

    println!("{:#?}", passport_str);

    let mut valid_counter: u32 = 0;

    for passport in passport_str {
        if Passport::parse(passport).is_valid() {
            valid_counter += 1;
        }
    }

    println!("The numer of valid passports is: {}", valid_counter);

}

#[derive(Debug)]
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
    pub fn parse(passport_str: String) -> Self {
        // the key:value pairs
        let pairs: Vec<&str> = passport_str.split(' ').collect();

        let mut passport: Passport = Passport::empty();

        for pair in pairs {
            let key_value: Vec<&str> = pair.split(':').collect();
            match key_value[0] {
                "byr" => passport.byr = Some(key_value[1].to_string()),
                "iyr" => passport.iyr = Some(key_value[1].to_string()),
                "eyr" => passport.eyr = Some(key_value[1].to_string()),
                "hgt" => passport.hgt = Some(key_value[1].to_string()),
                "hcl" => passport.hcl = Some(key_value[1].to_string()),
                "ecl" => passport.ecl = Some(key_value[1].to_string()),
                "pid" => passport.pid = Some(key_value[1].to_string()),
                "cid" => passport.cid = Some(key_value[1].to_string()),
                &_ => println!("key is empty") // do noting
            }
        }

        passport
    }

    // I am aware that this function is really ugly, im still a beginner, and should read the rust book, I know...
    pub fn is_valid(self) -> bool {
        let mut is_valid: bool = true;

        if !self.byr.is_some() {
            is_valid = false;
        }
        if !self.iyr.is_some() {
            is_valid = false;
        }
        if !self.eyr.is_some() {
            is_valid = false;
        }
        if !self.hgt.is_some() {
            is_valid = false;
        }
        if !self.hcl.is_some() {
            is_valid = false;
        }
        if !self.ecl.is_some() {
            is_valid = false;
        }
        if !self.pid.is_some() {
            is_valid = false;
        }
        /*
        if self.cid.is_some() {
            // no need to check, becuase cid is optional
        }
        */

        is_valid
    }

    fn empty() -> Self {
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
}
