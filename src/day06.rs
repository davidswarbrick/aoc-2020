use std::fs;

fn everyone_yes(input: &str) -> usize{
    let mut lines = input.lines();
    
    
    let mut everyone_yes = String::from(lines.next().unwrap());
    let mut everyone_yes_candidate = String::new(); 
    for other_person in lines {
        for c in other_person.chars(){
            if everyone_yes.contains(c){
                // If other person answered yes
                // then add this from "yes"s
                everyone_yes_candidate.push(c)
            }
        }
        everyone_yes = everyone_yes_candidate.clone();
        everyone_yes_candidate = String::new();
    }
    everyone_yes.len()

}


fn anyone_yes(input: &str) -> usize{
    let mut chars_seen = String::new();
    for c in input.chars(){
        if !chars_seen.contains(c){
            chars_seen.push(c);
        }
    }
    chars_seen.len()

}


fn input_from_file() -> usize{
    let filename = "./inputs/day06.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    let mut count = 0;
    let mut group = String::new();
    for line in contents.lines() {
        // Need to handle the last line, or just add another empty line
        if line.is_empty() {
            //count += anyone_yes(&group);
            println!("Group: {}",group);
            count += everyone_yes(&group);
            group = String::new();
        } else {
            group.push_str(line);
            group.push_str("\n");
        }
    }
    count
}

pub fn run() {
    let count_sum = input_from_file();
    println!("{}", count_sum);
}
