use std::fs;

fn input_from_file() -> Vec<i64> {
    let filename = "./inputs/day10-test.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    contents
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn next_jolt_steps(current_adapter: i64, next_few: &[i64]) -> Vec<i64> {
    let mut opts: Vec<i64> = vec![0, 0, 0];
    for i in 1..4 {
        let a = match next_few.contains(&(current_adapter + i)) {
            true => current_adapter + i,
            false => -1,
        };
        opts[i as usize - 1] = a;
    }
    opts
}
fn recurse_down(current_adapter: i64, mut explored: Vec<i64>, all_adapters: &[i64]) -> Vec<i64> {
    let opts = next_jolt_steps(current_adapter, all_adapters);
    for next_opt in opts.iter() {
        if *next_opt != -1 {
            explored = recurse_down(*next_opt, explored, all_adapters);
            explored.push(current_adapter);
        }
    }
    explored
}

fn vec_difference(b: &[i64], a: &[i64]) -> Vec<i64> {
    // return vec of b - a
    let mut diff = Vec::new();
    assert!(a.len() == b.len());
    for i in 0..a.len() - 1 {
        diff.push(b[i] - a[i]);
    }
    diff
}

fn zero_pad_left(a: &[i64], n: i64) -> Vec<i64> {
    let mut ret = Vec::new();
    let new_len = a.len() - n as usize;
    for i in 0..n as usize {
        ret.push(0);
    }
    for i in 0..new_len {
        ret.push(a[i]);
    }
    assert!(a.len() == ret.len());
    ret
}

fn zero_pad_right(a: &[i64], n: i64) -> Vec<i64> {
    let mut ret = Vec::new();
    let new_len = a.len() - n as usize;
    for i in 0..new_len {
        ret.push(a[i + n as usize]);
    }
    for i in 0..n as usize {
        ret.push(0);
    }
    assert!(a.len() == ret.len());
    ret
}

fn jolt_differences(adaptors: &[i64]) -> Vec<i64> {
    let a = zero_pad_right(adaptors, 1);
    let dist_to_next = vec_difference(&a, adaptors);
    println!("{:?}", dist_to_next);
    count_contiguous_ones(&dist_to_next)
}

fn count_contiguous_ones(v: &[i64]) -> Vec<i64> {
    // returns num contig 3s, then occurences of n number of threes, index = n
    let mut one_count = 0;
    let mut num_ones = vec![0, 0, 0, 0, 0];
    for i in 0..v.len() {
        if v[i] != 1 {
            num_ones[one_count as usize] += 1;
            one_count = 0;
        } else {
            one_count += 1;
        }
    }
    num_ones[one_count as usize] += 1;
    num_ones
}

pub fn run() {
    let mut v = input_from_file();
    v.sort();
    println!("{:?}", v);
    let three = jolt_differences(&v);
    println!("{:?}", three);
}
