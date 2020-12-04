use std::fs;

struct Passport {
    byr: i32,
    iyr: i32,
    eyr: i32,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}
//impl Passport {
//    fn valid(&self) -> bool {}
//}

fn valid_passport(m: &str) -> bool {
    if m.contains("byr:")
    && m.contains("iyr:")
    && m.contains("eyr:")
    && m.contains("hgt:")
    && m.contains("hcl:")
    && m.contains("ecl:")
    && m.contains("pid:") {
        return true;
    } 
    return false;    
}

//fn parse_passport(m: &str) -> Passport {}

fn input_from_file() -> i32 {
    let filename = "./inputs/day04.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    //let mut vec = Vec::new();
    let mut passport_candidate = String::new();
    let mut num_right = 0;
    for line in contents.lines() {
        // Need to handle the last line, or just add another empty line
        if line.is_empty(){
            let valid = valid_passport(&passport_candidate);
            println!("Valid passport: {}",valid);
            if valid {
                num_right +=1;
            }
            passport_candidate = String::new();
        } else {
            passport_candidate.push_str(line);
        }
    }
    num_right
}

pub fn run() {
    let passports = input_from_file();
    println!("{}", passports);
}
