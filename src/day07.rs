use regex::Regex;
use std::fs;

struct BagRule {
    colour: String,
    contains: Vec<(String, i32)>,
}

fn input_from_file() -> String {
    let filename = "./inputs/day07.txt";
    fs::read_to_string(filename).expect("No input.txt present")
}

fn rule_maker(s: &str) -> BagRule {
    lazy_static! {
        // Only want Regex to be compiled once
        static ref RE: Regex = Regex::new(r"((?P<num>\d+)+\s(?P<des>[\w]+\s[\w]+)\s+(?P<bag>bag[s]?[,.][\s\n]?))|(?P<no_other>no other bags.)").unwrap();
    }
    let v: Vec<&str> = s.split(" bags contain ").collect();
    let mut contains = Vec::new();
    for caps in RE.captures_iter(s) {
        let no_other_bags = caps.name("no_other");
        if no_other_bags == None {
            // no_other_bags didn't capture
            let num: i32 = caps["num"].parse::<i32>().unwrap();
            let des: String = String::from(&caps["des"]);
            contains.push((des, num));
        } else {
            // got "no other bags." thus for any colour 0 bags can be stored.
            contains.push((String::from("any"), 0));
        }
    }
    BagRule {
        colour: String::from(v[0]),
        contains: contains,
    }
}

fn find_outer_bag_colours(bag_colour: &str, rules: &Vec<BagRule>) -> Vec<String> {
    let mut allowed_outer_colours = Vec::new();
    let mut bag_col = String::from(bag_colour);
    let mut it = 0;
    loop {
        println!("Looking for {}", bag_col);
        for rule in rules {
            for (other_colour, _num) in &rule.contains {
                if other_colour == &bag_col && !allowed_outer_colours.contains(&rule.colour) {
                    println!("- {}", &rule.colour);
                    allowed_outer_colours.push(String::from(&rule.colour));
                }
            }
        }
        bag_col = match allowed_outer_colours.get(it) {
            Some(x) => String::from(x.as_str()),
            None => break,
        };
        it += 1;
    }
    allowed_outer_colours
}

fn get_rule<'a>(r: &'a str, rules: &'a Vec<BagRule>) -> Option<&'a BagRule> {
    let rule_id = rules.binary_search_by_key(&r, |a| &a.colour);
    if rule_id.is_ok() {
        return rules.get(rule_id.unwrap());
    } else {
        return None;
    }
}

fn recurse_down(
    colour: &str,
    number: i32,
    rules: &Vec<BagRule>,
    mut explored: Vec<(String, i32)>,
) -> Vec<(String, i32)> {
    let rule = match get_rule(&colour, rules) {
        Some(r) => r,
        None => return explored, // found bottom - this contains none
    };
    for (inner_bag, num) in &rule.contains {
        explored = recurse_down(&inner_bag, number * num, rules, explored);
        explored.push((inner_bag.to_string(), number * num));
    }
    explored
}

pub fn run() {
    let rule_strings = input_from_file();
    let mut rules = Vec::new();
    for line in rule_strings.lines() {
        let rule = rule_maker(line);
        //println!("{}-kind of bags contain {} * {}",rule.colour,rule.contains[0].0,rule.contains[0].1);
        rules.push(rule);
    }
    rules.sort_by_key(|a| a.colour.to_string());
    let mut explored = Vec::new();
    let cols = recurse_down("shiny gold", 1, &rules, explored);
    let mut total = 0;
    for c in cols {
        println!("{} * {}", c.0, c.1);
        total += c.1;
    }
    println!("{}", total);
}
