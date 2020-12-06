use std::fs;


fn single_group_count(input: &str) -> usize{
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
            count += single_group_count(&group);
            group = String::new();
        } else {
            group.push_str(line);
        }
    }
    count
}

pub fn run() {
    let count_sum = input_from_file();
    println!("{}", count_sum);
}
