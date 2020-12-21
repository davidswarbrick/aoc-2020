use std::collections::HashMap;
use std::fs;

struct Tile {
    id: i32,
    img: String,
    flip_x: bool,
    flip_y: bool,
    rotation: usize,
    matching_ids: [i32; 4],
}

impl Tile {
    pub fn new(tile_string: &str) -> Self {
        Self {
            id: tile_string.get(5..9).unwrap().parse().unwrap(),
            img: String::from(tile_string.get(11..tile_string.len() - 1).unwrap()),
            flip_x: false,
            flip_y: false,
            rotation: 0,
            matching_ids: [0, 0, 0, 0],
        }
    }
    fn print(&self) {
        println!("ID: {}", self.id);
        println!("{}", self.img);
    }
    fn get_column(&self, mut n: usize) -> Option<String> {
        let mut s = String::new();
        if self.flip_x {
            n = self.get_row(0).unwrap().len() - (n + 1);
        }

        //let mut it = self.img.lines();

        // ToDo: Implement something like this to decide iterator direction

        //if self.flip_y {
        //    it = self.img.lines().rev();
        //}

        // A match statement would be neatest, but these are diff. types.

        let mut it = match self.flip_y {
            false => self.img.lines(),
            true => self.img.lines().rev(),
        };
        for l in it {
            let c = l.get(n..n + 1);
            if c.is_some() && c.unwrap() != "\n" {
                s.push_str(c.unwrap());
            } else {
                return None;
            }
        }
        Some(s)
    }
    fn get_row(&self, mut n: usize) -> Option<String> {
        if self.flip_y {
            n = self.get_column(0).unwrap().len() - (n + 1);
        }
        // ToDo: Reverse strings if needed
        match self.img.lines().nth(n) {
            Some(x) => Some(x.to_string()),
            None => None,
        }
    }
    fn get_dims(&self) -> (usize, usize) {
        let x = self.get_row(0).unwrap().len();
        let y = self.get_column(0).unwrap().len();
        (x, y)
    }
    fn get_edges(&self) -> [String; 4] {
        let (x, y) = self.get_dims();
        let mut edges: [String; 4] = Default::default();
        edges[(self.rotation) % 4] = self.get_row(0).unwrap();
        edges[(self.rotation + 1) % 4] = self.get_column(x - 1).unwrap();
        edges[(self.rotation + 2) % 4] = self.get_row(y - 1).unwrap();
        edges[(self.rotation + 3) % 4] = self.get_column(0).unwrap();
        edges
    }
}

fn update_tile_map(mut tiles: HashMap<i32, Tile>, mut new_tile: Tile) -> HashMap<i32, Tile> {
    tiles.remove(&new_tile.id);
    for (_, tile) in tiles.iter_mut() {
        let edges = tile.get_edges();
        let new_edges = new_tile.get_edges();
        for i in 0..3 {
            if new_edges[i] == edges[(i + 2) % 4] {
                // matching edges (next to each other)
                new_tile.matching_ids[i] = tile.id;
                tile.matching_ids[(i + 2) % 4] = new_tile.id;
            }
        }
    }
    tiles.insert(new_tile.id, new_tile);
    tiles
}

fn input_from_file() -> HashMap<i32, Tile> {
    let filename = "./inputs/day20.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    let mut one_tile = String::new();
    let mut tiles = HashMap::new();

    for line in contents.lines() {
        if line.is_empty() {
            let t = Tile::new(&one_tile);
            tiles = update_tile_map(tiles, t);
            one_tile = String::new();
        } else {
            one_tile.push_str(line);
            one_tile.push_str("\n");
        }
    }
    tiles
}

pub fn run() {
    let mut tiles = input_from_file();
    // ToDo : Re-architect this to be memory safe. (Compilation fails)
    for tile in tiles.values_mut() {
        //println!("{} : {:?}", tile.id, tile.matching_ids);
        if tile.matching_ids == [0, 0, 0, 0] {
            tile.rotation += 1;
            tiles = update_tile_map(tiles, *tile);
        }
    }
}
