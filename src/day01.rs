use std::fs;

fn input_from_file() -> Vec<i32> {
    let filename = "./inputs/day01.txt";
    let contents = fs::read_to_string(filename).expect("No input.txt present");
    contents
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

pub fn run() {
    // Training data
    // let mut v = vec![1721,979,366,299,675,1456];
    let mut v = input_from_file();
    while let Some(a) = v.pop() {
        for b in &v {
            if a + b == 2020 {
                println!("Found 2020! {} * {} = {}", a, b, a * b);
            }
            for c in &v {
                if b != c {
                    if a + b + c == 2020 {
                        println!("Found 2020! {} * {} * {} = {}", a, b, c, a * b * c);
                    };
                }
            }
        }
    }
}
