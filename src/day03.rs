use std::fs;
use std::ops::Rem;

struct Map {
    _map : Vec<Vec<char>>,
}
impl Map {
    fn get_loc(&self, pos: (usize,usize)) -> &char {
        let (x,y) = pos;
        self._map.get(x).unwrap().get(y).unwrap()
    }
    fn is_tree(&self, pos: (usize,usize)) -> bool {
        let c = self.get_loc(pos);
        match c {
            '.' => false,
            '#' => true,
            _ => false,
        }
  
    }

}

fn input_from_file() -> Map {
    let filename = "./inputs/day03.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    let mut vec = Vec::new();
    for line in contents.lines() {
        let line_vec : Vec<char> = line.chars().collect();
        vec.push(line_vec);
    }
    let map = Map {_map: vec};
    map
}
pub fn run() {
    let map = input_from_file();
    println!("{}",map.is_tree((0,0)));
}
//fn input_from_file() -> Vec<Vec<char>> {
//    let filename = "./inputs/day03.txt";
//    let contents = fs::read_to_string(filename).expect("No input.txt present");
//    let mut vec = Vec::new();
//    for line in contents.lines() {
//        let line_vec : Vec<char> = line.chars().collect();
//        vec.push(line_vec);
//    }
//    vec
//}
//
//fn is_tree(map: &Vec<&Vec<char>>, pos: (usize,usize)) -> bool {
//    let (x,y) = pos;
//    if map.get(y) == None {
//        return false
//    } else {
//        let row = map.get(y).unwrap();
//        let mut c = 'N';
//        if row.get(x) == None{
//            let rem = x % row.len();
//            c = *row.get(rem).unwrap();
//        } else {
//            c = *row.get(x).unwrap();
//        }
//        match c {
//            '.' => false,
//            '#' => true,
//            _ => false,
//        }
//    }
//}
//
//
//fn traverse_map(map: Vec<Vec<char>>, dir: (usize,usize), start_pos: (usize,usize)) -> i32 {
//    let (mut x, mut y) = start_pos;
//    let (dir_x, dir_y) = dir;
//    let mut tree_count = 0;
//    while map.get(y) != None {
//        if is_tree(&map,(x,y)){
//            tree_count += 1;
//        }
//        x += dir_x;
//        y += dir_y;
//    }
//    tree_count 
//}
//
//
//
//pub fn run() {
//    let map = input_from_file();
//    let trees = traverse_map(map, (3,1), (0,0));
//    println!("Found {} trees in total",trees);
//}
