use std::fs;

struct Map {
    _map: Vec<Vec<char>>,
}
impl Map {
    fn get_loc(&self, pos: (usize, usize)) -> Option<&char> {
        let (x, y) = pos;
        if self._map.get(y) == None {
            return None;
        } else {
            match self._map.get(y).unwrap().get(x) {
                Some(x) => Some(x),
                None => self
                    ._map
                    .get(y)
                    .unwrap()
                    .get(x % self._map.get(y).unwrap().len()),
            }
        }
    }
    fn is_tree(&self, pos: (usize, usize)) -> Option<bool> {
        match self.get_loc(pos) {
            Some('.') => Some(false),
            Some('#') => Some(true),
            _ => None,
        }
    }
}

fn input_from_file() -> Map {
    let filename = "./inputs/day03.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    let mut vec = Vec::new();
    for line in contents.lines() {
        let line_vec: Vec<char> = line.chars().collect();
        vec.push(line_vec);
    }
    let map = Map { _map: vec };
    map
}

fn traverse_map(map: &Map, dir: (usize, usize), start_pos: (usize, usize)) -> i128 {
    let (mut x, mut y) = start_pos;
    let (dir_x, dir_y) = dir;
    let mut tree_count = 0;
    while map.is_tree((x, y)) != None {
        if map.is_tree((x, y)).unwrap() {
            tree_count += 1;
        }
        x += dir_x;
        y += dir_y;
    }
    tree_count
}

pub fn run() {
    let map = input_from_file();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut total: i128 = 1;
    for slope in slopes {
        let trees = traverse_map(&map, slope, (0, 0));
        total = total * trees;
    }
    println!("{}", total);
}
