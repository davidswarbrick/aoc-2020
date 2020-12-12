use std::fs;

#[derive(Clone)]
struct Map {
    _map: Vec<Vec<char>>,
}
impl Map {
    fn get_loc(&self, pos: (i64, i64)) -> Option<&char> {
        let (x_i, y_i) = pos;
        if x_i < 0 || y_i < 0 {
            return None;
        };
        let x = x_i as usize;
        let y = y_i as usize;
        
        match self._map.get(y) {
            None => return None,
            Some(y) => return y.get(x)
        }
    }
    fn set_loc(&mut self, pos: (i64,i64), val: char) {
        let (x,y) = pos;
        self._map[y as usize][x as usize] = val;
    }
    fn num_adjacent_filled(&self, pos: (i64,i64)) -> i64 {
        let other_pos : [(i64,i64); 8] = [(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1), (1,1)];
        let mut occupied = 0;
        let (p_x, p_y) = pos;
        for (x,y) in other_pos.iter() {
            match self.get_loc((p_x + x, p_y + y)) {
                Some('#') => occupied +=1,
                _ => (),
            }
        }
        occupied
    }
    fn size(&self) -> (i64,i64) {
        (self._map[0].len() as i64,self._map.len() as i64)
    }
    fn num_occupied(&self) -> i64 {
        let (x_max, y_max) = self.size();
        let mut occupied = 0;
        for y in 0 .. y_max {
            for x in 0 .. x_max {
                match self.get_loc((x,y)) {
                    Some('#') => occupied +=1,
                    _ => ()
                }
            }
        }
        occupied
    }
}

fn input_from_file() -> Map {
    let filename = "./inputs/day11.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    let mut vec = Vec::new();
    for line in contents.lines() {
        let line_vec: Vec<char> = line.chars().collect();
        vec.push(line_vec);
    }
    let map = Map { _map: vec };
    map
}


fn generate_new_map(m: &Map) -> (Map,i64){
    let (x_max, y_max) = m.size();
    let mut m_new = m.clone();
    let mut changes = 0;
    for y in 0 .. y_max {
        for x in 0 .. x_max {
            let occ = m.num_adjacent_filled((x,y));
            let mut val = m.get_loc((x,y)).unwrap().clone();
            if val == 'L' && occ == 0 {
                val = '#';
                changes +=1;
            } else if val == '#' && occ >= 4 {
                val = 'L';
                changes +=1;
            }
            m_new.set_loc((x,y),val);
        }
    }
    (m_new,changes)
}


//fn traverse_map(map: &Map, dir: (usize, usize), start_pos: (usize, usize)) -> i128 {
//    let (mut x, mut y) = start_pos;
//    let (dir_x, dir_y) = dir;
//    let mut tree_count = 0;
//    while map.is_tree((x, y)) != None {
//        if map.is_tree((x, y)).unwrap() {
//            tree_count += 1;
//        }
//        x += dir_x;
//        y += dir_y;
//    }
//    tree_count
//}

pub fn run() {
    let mut map = input_from_file();
    loop {
        let (new_map, changes) = generate_new_map(&map);
        println!("Occupied: {}, Changes: {}",new_map.num_occupied(),changes); 
        if changes == 0 {
            break
        }
        map = new_map;
    }
}
