use std::fs;

fn input_from_file() -> Vec<i64> {
    let filename = "./inputs/day09.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    contents
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn ok_value(vec_slice: &[i64], value: &i64) -> bool {
    let mut possible_values: Vec<i64> = Vec::new();
    for j in 0..vec_slice.len() - 1 {
        for k in j + 1..vec_slice.len() {
            possible_values.push(vec_slice.get(j).unwrap() + vec_slice.get(k).unwrap());
        }
    }
    //println!("{}: Poss values {:?}",value,possible_values);
    possible_values.contains(value)
}

fn find_contiguous(vec_slice: &[i64], value: &i64) -> (usize, usize) {
    for j in 1..vec_slice.len() {
        let mut k = j - 1;
        loop {
            //println!("Searching k: {} j: {}",k,j);
            let mut total: i64 = 0;
            for prev_val in vec_slice[k..j].iter() {
                total += prev_val
            }
            if total == *value {
                return (k, j);
            } else {
                if k == 0 {
                    break;
                } else {
                    k -= 1;
                }
            }
        }
    }
    (0, 0)
}

pub fn run() {
    let v = input_from_file();
    for i in 25..v.len() {
        let value = v.get(i).unwrap();
        let slice = &v[i - 25..i];
        //println!("{} : {:?}",value,slice);
        if !ok_value(slice, value) {
            println!("{}", value);
            let (start, end) = find_contiguous(&v, value);
            println!("{} {}", start, end);
            let mut x = v[start..end].to_vec();
            x.sort();
            let a = x[0];
            let b = x[x.len() - 1];
            println!("Min: {} Max: {}", x[0], x[x.len() - 1]);
            println!("{}", a + b);
            break;
        }
    }
}
