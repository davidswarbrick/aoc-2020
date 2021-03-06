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

fn parse_single_val<'a>(long_str: &'a str, prefix: &'a str) -> &'a str {
    // Find the prefix, add 4 for the label name, split at space and return
    long_str
        .split_at(long_str.find(prefix).unwrap() + 4)
        .1
        .split_whitespace()
        .next()
        .unwrap()
}

impl Passport {
    pub fn new(passport_string: &str) -> Self {
        Self {
            byr: parse_single_val(passport_string, "byr:")
                .parse::<i32>()
                .unwrap(),
            iyr: parse_single_val(passport_string, "iyr:")
                .parse::<i32>()
                .unwrap(),
            eyr: parse_single_val(passport_string, "eyr:")
                .parse::<i32>()
                .unwrap(),
            hgt: String::from(parse_single_val(passport_string, "hgt:")),
            hcl: String::from(parse_single_val(passport_string, "hcl:")),
            ecl: String::from(parse_single_val(passport_string, "ecl:")),
            pid: String::from(parse_single_val(passport_string, "pid:")),
            cid: String::from("Empty"),
            //cid: String::from(parse_single_val(&passport_string,"cid")),
        }
    }
    fn validate(&self) -> bool {
        let mut valid = false;
        // Check int values for byr, iyr, eyr
        valid = self.byr >= 1920 && self.byr <= 2002;
        valid &= self.iyr >= 2010 && self.iyr <= 2020;
        valid &= self.eyr >= 2020 && self.eyr <= 2030;
        valid &= match self.ecl.as_str() {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            _ => false,
        };

        // pid
        valid &= self.pid.chars().collect::<Vec<char>>().len() == 9;
        valid &= self.pid.parse::<i64>().is_ok();

        // hcl
        valid &= self.hcl.chars().collect::<Vec<char>>().len() == 7;
        let mut it = self.hcl.chars();
        valid &= it.next().unwrap() == '#';
        for c in it {
            valid &= c.is_numeric()
                || match c {
                    'a' => true,
                    'b' => true,
                    'c' => true,
                    'd' => true,
                    'e' => true,
                    'f' => true,
                    _ => false,
                };
        }
        // hgt
        valid &= self.hgt.ends_with("cm") || self.hgt.ends_with("in");
        if self.hgt.ends_with("cm") {
            let num = self.hgt.replace("cm", "").parse::<i64>().unwrap();
            valid &= num >= 150 && num <= 193;
        }
        if self.hgt.ends_with("in") {
            let num = self.hgt.replace("in", "").parse::<i64>().unwrap();
            valid &= num >= 59 && num <= 76;
        }
        valid
    }
}

fn valid_passport(m: &str) -> bool {
    if m.contains("byr:")
        && m.contains("iyr:")
        && m.contains("eyr:")
        && m.contains("hgt:")
        && m.contains("hcl:")
        && m.contains("ecl:")
        && m.contains("pid:")
    {
        return true;
    }
    return false;
}

fn input_from_file() -> Vec<Passport> {
    let filename = "./inputs/day04.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    let mut vec = Vec::new();
    let mut passport_candidate = String::new();
    for line in contents.lines() {
        // Need to handle the last line, or just add another empty line
        if line.is_empty() {
            let valid = valid_passport(&passport_candidate);
            if valid {
                let p = Passport::new(&passport_candidate);
                if p.validate() {
                    vec.push(p)
                }
            }
            passport_candidate = String::new();
        } else {
            // Input a space instead of the newline to separate variables
            passport_candidate.push_str(" ");
            passport_candidate.push_str(line);
        }
    }
    vec
}

pub fn run() {
    let passports = input_from_file();
    println!("{}", passports.len());
}
