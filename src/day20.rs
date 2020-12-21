use std::collections::HashMap;
use std::fs;
struct Tile {
    id: i32,
    img: String,
    flip_x: bool,
    flip_y: bool,
    rotation: usize,
    matching_ids: [i32; 4],
    dims: (usize,usize),
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
            dims : (0,0),
        }

    }
    fn set_dims(&mut self) -> (usize, usize) {
        let x = self.img.lines().next().unwrap().len();
        self.dims = (x,x.clone());
        self.dims
    }

    fn get_dims(&mut self) -> (usize,usize) {
        return match self.dims {
            (0,0) => self.set_dims(),
            x => x
        }
    }
    fn print(&self) {
        println!("ID: {} x: {} y: {} rotation: {}", self.id,self.flip_x,self.flip_y,self.rotation);
        //println!("{}", self.img);
    }
    fn get_column(&mut self, mut n: usize) -> Option<String> {
        let mut s = String::new();
        if self.flip_x {
            let (row_len, _) = self.get_dims();
            n = row_len - (n + 1);
        }
        let it: Vec<&str> = match self.flip_y {
            false => self.img.lines().collect(),
            true => self.img.lines().rev().collect(),
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
    fn get_row(&mut self, mut n: usize) -> Option<String> {
        if self.flip_y {
            let (_, col_height) = self.get_dims();
            n = col_height - (n + 1);
        }
        // ToDo: Reverse strings if needed
        let row_string = match self.img.lines().nth(n) {
            Some(x) => x.to_string(),
            None => return None,
        };
        match self.flip_x {
            false => Some(row_string),
            true => Some(row_string.chars().rev().collect::<String>())
        }
    }
    fn get_edges(&mut self) -> [String; 4] {
        let (x, y) = self.get_dims();
        let mut edges: [String; 4] = Default::default();
        edges[(self.rotation) % 4] = self.get_row(0).unwrap();
        edges[(self.rotation + 1) % 4] = self.get_column(x - 1).unwrap();
        edges[(self.rotation + 2) % 4] = self.get_row(y - 1).unwrap();
        edges[(self.rotation + 3) % 4] = self.get_column(0).unwrap();
        edges
    }
}

fn update_tile_map(mut tiles: HashMap<i32, Tile>, tile_id : &i32) -> HashMap<i32, Tile> {
    let mut new_tile = tiles.remove(tile_id).unwrap();
    'all_tiles: for tile in tiles.values_mut() {
        let edges = tile.get_edges();

        for x_flip in 0 ..=1 { 
            new_tile.flip_x = x_flip ==1 ;

            for y_flip in 0 ..=1 { 
                new_tile.flip_y = y_flip == 1;

                for rotation in 0..=3 {
                    new_tile.rotation = rotation; 
                    // new_tile.print();
                    let new_edges = new_tile.get_edges();

                    // For this configuration of edges, check all for match.
                    for i in 0..3 {
                        if new_edges[i] == edges[(i + 2) % 4] {
                            // Record matching edges (next to each other)
                            new_tile.matching_ids[i] = tile.id;
                            tile.matching_ids[(i + 2) % 4] = new_tile.id;
                            
                            // As a match has been found, stop altering this tile
                            break 'all_tiles;
                        }
                    }
                }
            }
        }
    }
    tiles.insert(new_tile.id, new_tile);
    tiles
}

fn input_from_file() -> (HashMap<i32, Tile>, Vec<i32>) {
    let filename = "./inputs/day20.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    let mut one_tile = String::new();
    let mut tile_ids = Vec::new();
    let mut tiles : HashMap<i32, Tile>  = HashMap::new();

    for line in contents.lines() {
        if line.is_empty() {
            let t = Tile::new(&one_tile);
            tile_ids.push(t.id);
            tiles.insert(t.id,t);
            one_tile = String::new();
        } else {
            one_tile.push_str(line);
            one_tile.push_str("\n");
        }
    }
    (tiles,tile_ids)
}

pub fn run() {
    let (mut tiles, tile_ids) = input_from_file();
    for id in tile_ids {
        tiles = update_tile_map(tiles,&id);
        println!("{} : {:?}",id,tiles[&id].matching_ids);
    }
}
