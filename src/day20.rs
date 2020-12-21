use std::collections::HashMap;
use std::fs;
struct Tile {
    id: i32,
    img: String,
    flip_x: bool,
    flip_y: bool,
    rotation: usize,
    matching_ids: [i32; 4],
    dims: (usize, usize),
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
            dims: (10, 10),
        }
    }
    fn set_dims(&mut self) -> (usize, usize) {
        let x = self.img.lines().next().unwrap().len();
        self.dims = (x, x.clone());
        self.dims
    }

    fn get_dims(&mut self) -> (usize, usize) {
        return match self.dims {
            (0, 0) => self.set_dims(),
            x => x,
        };
    }
    fn print(&self) {
        println!(
            "{} x: {:6} y: {:6} rotation: {} - Matches: {:?} ",
            self.id, self.flip_x, self.flip_y, self.rotation, self.matching_ids
        );
        //println!("{}", self.img);
    }
    fn get_column(&self, mut n: usize) -> Option<String> {
        let mut s = String::new();
        if self.flip_x {
            let (row_len, _) = self.dims;
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
    fn get_row(&self, mut n: usize) -> Option<String> {
        if self.flip_y {
            let (_, col_height) = self.dims;
            n = col_height - (n + 1);
        }
        // ToDo: Reverse strings if needed
        let row_string = match self.img.lines().nth(n) {
            Some(x) => x.to_string(),
            None => return None,
        };
        match self.flip_x {
            false => Some(row_string),
            true => Some(row_string.chars().rev().collect::<String>()),
        }
    }
    fn get_edges(&self) -> [String; 4] {
        let (x, y) = self.dims;
        let mut edges: [String; 4] = Default::default();
        edges[(self.rotation) % 4] = self.get_row(0).unwrap();
        edges[(self.rotation + 1) % 4] = self.get_column(x - 1).unwrap();
        edges[(self.rotation + 2) % 4] = self.get_row(y - 1).unwrap();
        edges[(self.rotation + 3) % 4] = self.get_column(0).unwrap();
        edges
    }
}



fn find_matches(tiles: &HashMap<i32,Tile>, edge: &str) -> Vec<(i32,usize)> {
   let mut matches = Vec::new();
   for t in tiles.values() {
        let ed = t.get_edges();
        for i in 0..=3 {
            // check if edge matches e or e in reverse
            let e = &ed[i];
            if edge == e || edge == e.chars().rev().collect::<String>()  {
                matches.push((t.id,i));
            }
        }
   } 
   matches
}




fn update_tile_map(mut tiles: HashMap<i32, Tile>, tile_id: &i32) -> HashMap<i32, Tile> {
    let mut new_tile = tiles.remove(tile_id).unwrap();
    let mut config_match_ids = Vec::new();
    'tile_config: for x_flip in 0..=1 {
        new_tile.flip_x = x_flip == 1;

        for y_flip in 0..=1 {
            new_tile.flip_y = y_flip == 1;

            for rotation in 0..=3 {
                new_tile.rotation = rotation;
                // new_tile.print();
                let new_edges = new_tile.get_edges();

                // For this configuration of edges, check all for match.

                for tile in tiles.values_mut() {
                    let edges = tile.get_edges();

                    'edge_match: for i in 0..=3 {
                        if new_edges[i] == edges[(i + 2) % 4] {
                            // Record matching edges (next to each other)
                            new_tile.matching_ids[i] = tile.id;
                            tile.matching_ids[(i + 2) % 4] = new_tile.id;

                            // As a match has been found for this edge,
                            // it should not match any other edges
                            break 'edge_match;
                        }
                    }
                }
                // All tiles checked in this configuration
                if !new_tile.matching_ids.contains(&0) {
                    //println!("Full match: {}: {:?}",new_tile.id,new_tile.matching_ids);
                    break 'tile_config;
                } else {
                    // Record matches, and reset to zero
                    config_match_ids.push(new_tile.matching_ids);
                    for i in 0..=3 {
                        let t = new_tile.matching_ids[i];
                        if let Some(tile) = tiles.get_mut(&t) {
                            tile.matching_ids[(i + 2) % 4] = 0;
                        }
                    }   
                    new_tile.matching_ids = [0, 0, 0, 0];
                }
            }
        }
    }
    if new_tile.matching_ids == [0, 0, 0, 0] {
        // Didn't find an orientation that matched all sides
        // choose orientation that had most matches

        // Count how many edges were matched
        let num_not_zero: Vec<usize> = config_match_ids
            .iter()
            .map(|x| x.iter().filter(|i| i != &&0).count())
            .collect();

        // Find index of most matches
        let mut sorted_num = num_not_zero.to_vec();
        sorted_num.sort();
        let index = num_not_zero
            .iter()
            .position(|&x| x == *sorted_num.last().unwrap())
            .unwrap();

        // Mask index to set tile appropriately.
        new_tile.flip_x = (index & 0b1000) != 0;
        new_tile.flip_y = (index & 0b0100) != 0;
        new_tile.rotation = index & 0b0011;

        // Reflect tile config in match ids of this and the matched tiles
        new_tile.matching_ids = config_match_ids[index];
        for i in 0..=3 {
            let t = new_tile.matching_ids[i];
            if let Some(tile) = tiles.get_mut(&t) {
                tile.matching_ids[(i + 2) % 4] = new_tile.id;
                //println!("{}: {:?} \n {}:{:?}",new_tile.id,new_tile.matching_ids, tile.id,tile.matching_ids);
            }
        }
    }
    tiles.insert(new_tile.id, new_tile);
    tiles
}

fn map_valid(map: &HashMap<i32, Tile>) -> bool {
    for (id, tile) in map {
        //println!("Validating {}: {:?}",id,tile.matching_ids);
        for i in 0..=3 {
            let t = tile.matching_ids[i];
            if let Some(other_tile) = map.get(&t) {
                // If tile exists (ie non-zero), should be reciprocal

                //println!(" checking exists in {}: {:?}",
                //    other_tile.id, other_tile.matching_ids
                //);
                match other_tile.matching_ids[(i + 2) % 4] == tile.id {
                    false => return false,
                    true => (),
                };
            }
        }
    }
    true
}

fn input_from_file() -> (HashMap<i32, Tile>, Vec<i32>) {
    let filename = "./inputs/day20.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    let mut one_tile = String::new();
    let mut tile_ids = Vec::new();
    let mut tiles: HashMap<i32, Tile> = HashMap::new();

    for line in contents.lines() {
        if line.is_empty() {
            let t = Tile::new(&one_tile);
            tile_ids.push(t.id);
            tiles.insert(t.id, t);
            one_tile = String::new();
        } else {
            one_tile.push_str(line);
            one_tile.push_str("\n");
        }
    }
    (tiles, tile_ids)
}

pub fn run() {
    let (mut tiles, tile_ids) = input_from_file();
    //println!("Map valid : {}", map_valid(&tiles));
    ////let id: i32 = 1579;
    //for id in tile_ids {
    //    tiles = update_tile_map(tiles, &id);
    //    tiles[&id].print();
    //}
    //println!("Map valid : {}", map_valid(&tiles));
    let mut edges : HashMap<i32,i32> = HashMap::new();
    for id in tile_ids {
        let ed = &tiles[&id].get_edges();
        for i in 0..=3 {
            // check if edge matches e or e in reverse
            let e = &ed[i];
            let matches = find_matches(&tiles,e);
            if matches.len() == 1 {
                *edges.entry(id).or_insert(0) += 1; 
            }
        }
    }
    let mut total: i64 = 1;
    for (id, matches) in edges {
        if matches == 2 {
            println!("{}",id);
            total *= id as i64;
        } 
    }
    println!("{}",total);
}
