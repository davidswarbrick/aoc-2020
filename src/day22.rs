use std::collections::HashSet;
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
    (one, two)
}

fn play_round(mut one: Vec<i64>, mut two: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
    let p1 = one.remove(0);
    let p2 = two.remove(0);
    if p1 > p2 {
        // player 1 wins
        one.push(p1);
        one.push(p2);
    } else if p2 > p1 {
        // player 2 wins
        two.push(p2);
        two.push(p1);
    }
    (one, two)
}

fn play_round_recursive_combat(mut one: Vec<i64>, mut two: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
    println!("1: len: {} : {:?}", one.len(), one);
    println!("2: len: {} : {:?}", two.len(), two);
    let p1 = one.remove(0);
    let p2 = two.remove(0);
    let mut player_one_wins = false;
    if p1 as usize <= one.len() && p2 as usize <= two.len() {
        // round of recursive combat
        println!("Recursing");
        let mut l_one = one[..p1 as usize].to_vec();
        let mut l_two = two[..p2 as usize].to_vec();
        player_one_wins = play_game_recursive_combat(l_one, l_two);
    } else {
        // normal round of highest card wins
        player_one_wins = p1 > p2;
    }

    if player_one_wins {
        println!("Player 1 wins");
        one.push(p1);
        one.push(p2);
    } else {
        println!("Player 2 wins");
        two.push(p2);
        two.push(p1);
    }

    (one, two)
}

fn calc_score(deck: &Vec<i64>) -> i64 {
    let mut total = 0;
    let mut multiplier = 1;
    for i in deck.iter().rev() {
        total += i * multiplier;
        multiplier += 1;
    }
    total
}
fn play_game_recursive_combat(mut start_one: Vec<i64>, mut start_two: Vec<i64>) -> bool {
    let mut prev_one = HashSet::new();
    let mut prev_two = HashSet::new();
    println!("Starting Game of Recursive Combat");
    while !(&start_one.len() == &0 || &start_two.len() == &0) {
        if prev_one.contains(&start_one) || prev_two.contains(&start_two) {
            return true;
        } else {
            prev_one.insert(start_one.to_vec());
            prev_two.insert(start_two.to_vec());
        }

        let (a, b) = play_round_recursive_combat(start_one, start_two);

        start_one = a;
        start_two = b;
    }
    let score_1 = calc_score(&start_one);
    let score_2 = calc_score(&start_two);
    println!("Player 1 score: {}", score_1);
    println!("Player 2 score: {}", score_2);
    score_1 > score_2
}

pub fn run() {
    let (mut one, mut two) = input_from_file();
    play_game_recursive_combat(one, two);
}
