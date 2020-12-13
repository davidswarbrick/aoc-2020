use std::fs;

fn input_from_file() -> (i64, Vec<(i64,i64)>) {
    let filename = "./inputs/day13.txt";
    let f = fs::read_to_string(filename).expect("No input.txt present");
    let mut lines = f.lines();
    let timestamp = lines.next().unwrap().parse::<i64>().unwrap();

    let mut buses = Vec::new(); 
    let mut index = 0;
    for b in lines.next().unwrap().split(',') {
        if b != "x" {
            buses.push((b.parse::<i64>().unwrap(),index));
        }
        index+=1;
    }
    println!("{:?}",buses);
    (timestamp, buses)
}

fn time_to_next_bus(id:&i64, timestamp:i64) -> i64 {
    let rem = timestamp % id;
    id - rem 
}

fn buses_in_order(sched:&Vec<(i64,i64)>) -> i64 {
    let mut sched_it = sched.iter();
    let (zero_id, _ ) = sched_it.next().unwrap();
    let mut timestamp = 0;
    let mut all_in_order = true;
    let mut multiplier = 1;
    loop {
        timestamp = multiplier * zero_id;
        for (bus,index) in sched_it {
            println!("Timestamp {} Checking {},{}",timestamp,bus,index);
            if time_to_next_bus(&bus,timestamp) != *index {
                all_in_order = false;
                break
            }
        }
        if all_in_order {
            break
        } else {
            sched_it = sched.iter();
            let _ = sched_it.next();
            multiplier+=1;
            all_in_order = true;
        }
    }

    //for (bus,index) in sched {
    //    println!("{} {}",bus,index);
    //}
    timestamp
}

pub fn run() {
    let (timestamp, buses) = input_from_file();
    let x = buses_in_order(&buses);
    println!("{}",x);
    //let mut next_bus = 0;
    //let mut wait_time = 100000;
    //for bus in buses.iter() {
    //    let wait_for_this_bus = time_to_next_bus(bus,timestamp);
    //    if wait_for_this_bus < wait_time {
    //        next_bus = *bus;
    //        wait_time = wait_for_this_bus;
    //    }
    //}
    //println!("Bus {}, arriving in {} minutes, Multiplied: {}",next_bus, wait_time, next_bus * wait_time );
}
