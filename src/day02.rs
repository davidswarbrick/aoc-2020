use std::fs;

fn input_from_file() -> String {
    let filename = "./inputs/day02.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    contents
}

fn line_checker(s: &str) -> bool {
    let v: Vec<&str> = s.split(|c| c == ' ' || c == '-' || c == ':').collect();
    //let lower = v.next().unwrap();
    let low = &v[0].parse::<i32>().unwrap();
    let high = &v[1].parse::<i32>().unwrap();
    let c = &v[2].chars().next().unwrap();
    let pass = &v[4];
    valid_password(*low, *high, *c, pass)
}

fn valid_password(low: i32, high: i32, character: char, password: &str) -> bool {
    // Part 1
    //let n: Vec<&str> = password.matches(character).collect();
    //(n.len() as i32) < high + 1 && (n.len() as i32) >= low
    // Part 2
    let mut low_match = false;
    let mut high_match = false;
    for (n, _) in password.match_indices(character) {
        low_match = (n as i32 + 1 == low) || low_match;
        high_match = (n as i32 + 1 == high) || high_match;
    }
    high_match ^ low_match
}

pub fn run() {
    let passwords = input_from_file();
    let mut i = 0;
    for line in passwords.lines() {
        let valid = line_checker(line);
        if valid {
            println!("Password Valid {}", line);
            i += 1;
        }
    }
    println!("Number valid: {}", i);
}
