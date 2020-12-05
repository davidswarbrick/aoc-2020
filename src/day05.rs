use std::fs;

fn parse_boarding_pass(bp: &str) -> i64{
    let mut row = 127;
    let mut col = 7;
    let mut mask = 0x40;
    let mut cmask = 0x40 << 3; 
    for c in bp.chars() {
        //println!("{} row: {} mask: {}, col: {}, cmask: {}",c,row,mask,col,cmask);
        row = match c {
            'F' => row ^ mask,
            'B' => row | mask,
            _ => row
        };
        //mask = match c { 
        //    'F' => mask >> 1,
        //    'B' => mask >> 1,
        //    _ => mask
        //};

        col = match c {
            'R' => col | cmask,
            'L' => col ^ cmask,
            _ => col
        };
        
        mask >>= 1;
        cmask >>=1;
        //cmask = match c { 
        //    'R' => cmask >> 1,
        //    'L' => cmask >> 1,
        //    _ => mask
        //};
        //println!("{} row: {} , col: {}",c,row,col);
    }
    row*8 + col
}



pub fn run() {
    let filename = "./inputs/day05.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    let mut max_id = 0;
    for line in contents.lines(){
        let seat_id = parse_boarding_pass(&line);
        if seat_id > max_id {
            max_id = seat_id;
        }
    }
    println!("Maximum Seat ID: {}",max_id);
}
