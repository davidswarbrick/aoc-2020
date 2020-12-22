use std::fs;

fn input_from_file() -> (Vec<i64>, Vec<i64>) {
    let filename = "./inputs/day22.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    let mut one = Vec::new();
    let mut two = Vec::new();
    let mut player_one = true;
    for line in contents.lines() {
        if line.is_empty() || line == "Player 1:" {
            player_one = true;
        } else if line == "Player 2:" {
            player_one = false;
        } else {
            if player_one {
                one.push(line.parse::<i64>().unwrap());
            } else {
                two.push(line.parse::<i64>().unwrap());
            }
        }
    }
    (one,two)
}

fn play_round(mut one:Vec<i64>, mut two:Vec<i64>) -> (Vec<i64>, Vec<i64>) {
    let mut p1 = one.remove(0); 
    let mut p2 = two.remove(0); 
    if p1 > p2 {
        // player 1 wins
        one.push(p1);
        one.push(p2);
    } else if  p2 > p1 {
        // player 2 wins
        two.push(p2);
        two.push(p1);
    }
    (one,two)
}
fn calc_score(deck : &Vec<i64>) -> i64 {
    let mut total = 0;
    let mut multiplier = 1;
    for i in deck.iter().rev() {
        total += i * multiplier;
        multiplier +=1;
    } 
    total
}


pub fn run() {
    let (mut one,mut two) = input_from_file();
    while !( &one.len() == &0 || &two.len() == &0) {
        let (a,b)  = play_round(one,two);
        one = a;
        two = b;
    }
    println!("one: {}",calc_score(&one));
    println!("two: {:?}",two);
}

