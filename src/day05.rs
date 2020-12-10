use std::fs;

fn parse_boarding_pass(bp: &str) -> i64 {
    let mut row = 127;
    let mut col = 7;
    let mut mask = 0x40;
    let mut cmask = 0x40 << 3;
    for c in bp.chars() {
        row = match c {
            'F' => row ^ mask,
            'B' => row | mask,
            _ => row,
        };
        col = match c {
            'R' => col | cmask,
            'L' => col ^ cmask,
            _ => col,
        };
        mask >>= 1;
        cmask >>= 1;
    }
    (row << 3) + col
}

pub fn run() {
    let filename = "./inputs/day05.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    let mut max_id = 0;
    let mut all_ids = Vec::new();
    for line in contents.lines() {
        let seat_id = parse_boarding_pass(&line);
        all_ids.push((seat_id >> 3, seat_id & 0x007));
        if seat_id > max_id {
            max_id = seat_id;
        }
    }
    // Not the neatest way around it, but this does a quick and dirty job of
    // checking if a seat is in the found boarding IDs or not.
    // Minimum seat was 35, max was 885, so we know it's in row 4-110
    // There must be a nicer way using the fact that the ID is [row][col] (in binary)
    for x in 4..111 {
        for y in 0..8 {
            if !all_ids.contains(&(x, y)) {
                println!("Found Empty ID {}", (x << 3) + y);
            }
        }
    }
    println!("Maximum Seat ID: {}", max_id);
}
