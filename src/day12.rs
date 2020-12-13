use std::fs;

fn input_from_file() -> String {
    let filename = "./inputs/day12.txt";
    fs::read_to_string(filename).expect("No input.txt present")
}

fn forward(pos: &(i64, i64, i64), num: i64) -> (i64, i64, i64) {
    let (mut x, mut y, mut dir) = pos.clone();
    match dir {
        90 => y += num,
        270 => y -= num,
        0 => x += num,
        180 => x -= num,
        _ => (),
    }
    (x, y, dir)
}

fn move_ship(action: &str, pos: &(i64, i64, i64)) -> (i64, i64, i64) {
    let (mut x, mut y, mut dir) = pos.clone();
    let (c, n) = action.split_at(1);
    let num = n.parse::<i64>().unwrap();

    if c == "F" {
        return forward(&pos, num);
    };
    match c {
        "N" => y += num,
        "S" => y -= num,
        "E" => x += num,
        "W" => x -= num,
        "R" => dir -= num,
        "L" => dir += num,
        _ => (),
    };
    if dir >= 360 {
        dir -= 360;
    } else if dir < 0 {
        dir += 360;
    }
    (x, y, dir)
}

fn to_waypoint(pos: &(i64, i64), wayp: &(i64, i64)) -> ((i64, i64), (i64, i64)) {}

fn move_ship_and_waypoint(
    action: &str,
    pos: &(i64, i64),
    wayp: &(i64, i64),
) -> ((i64, i64), (i64, i64)) {
    let (mut w_x, mut w_y) = wayp.clone();
    let (c, n) = action.split_at(1);
    let num = n.parse::<i64>().unwrap();

    if c == "F" {
        return to_waypoint(&pos, num);
    };
    match c {
        "N" => w_y += num,
        "S" => w_y -= num,
        "E" => w_x += num,
        "W" => w_x -= num,
        _ => (),
    };
    (pos, (w_x, w_y))
}
pub fn run() {
    let i = input_from_file();
    let mut ship = (0, 0, 0);
    for cmd in i.lines() {
        ship = move_ship(cmd, &ship);
        println!("{:?}", ship);
    }
    let (x, y, _) = ship;
    println!("Manhattan distance: {}", x.abs() + y.abs());
}
