use regex::Regex;
use std::fs;

fn input_from_file() -> String {
    let filename = "./inputs/day08.txt";
    fs::read_to_string(filename).expect("No input.txt present")
}

fn processor(s: &str) -> i64 {
    lazy_static! {
        // Only want Regex to be compiled once
        static ref RE: Regex = Regex::new(r"(?P<cmd>\w+)\s(?P<num>[+|-]\d+)").unwrap();
    }
    let mut acc = 0;
    let mut i: i64 = 0;
    let mut prev_visited = Vec::new();
    let cmds: Vec<regex::Captures> = RE.captures_iter(s).collect();
    println!("{:?}", cmds);
    let mut prev_switched = Vec::new();
    let mut switch_has_occurred = true;
    loop {
        if prev_visited.contains(&i) {
            println!("Switched: {} out of {}", prev_switched.len(), cmds.len());
            prev_visited = Vec::new();
            switch_has_occurred = false;
            acc = 0;
            i = 0;
            //break
        }
        prev_visited.push(i);
        let caps = match cmds.get(i as usize) {
            Some(x) => x,
            None => break,
        };
        let mut cmd = caps.name("cmd").unwrap().as_str();
        if (cmd == "jmp" || cmd == "nop") && !switch_has_occurred && !prev_switched.contains(&i) {
            cmd = match cmd {
                "nop" => "jmp",
                "jmp" => "nop",
                _ => "nop",
            };
            switch_has_occurred = true;
            prev_switched.push(i);
        }

        let num = caps.name("num").unwrap().as_str().parse::<i64>().unwrap();
        let (jump, acc_diff) = match cmd {
            "acc" => (1, num),
            "jmp" => (num, 0),
            "nop" => (1, 0),
            _ => (1, 0),
        };
        //println!("i:{}  cmd:{}  num:{}  acc:{}",i,cmd,num,acc);
        //println!("   jump:{}  acc_diff:{}",jump,acc_diff);
        i += jump;
        acc += acc_diff;
    }
    acc
}

pub fn run() {
    let cmds = input_from_file();
    let acc = processor(&cmds);
    println!("{}", acc);
}
