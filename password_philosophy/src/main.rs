use std::{fs, process};

fn main() {
    let file = fs::read_to_string("b.in").unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        process::exit(1);
    });

    let contents = file.lines();
    let mut pass_count = 0;

    for line in contents {
        let line_split: Vec<&str> = line.split(|s| s == '-' || s == ' ' || s == ':').collect();
        // println!("{:?}", line_split);

        let lower: usize = line_split[0].parse().unwrap();
        let upper: usize = line_split[1].parse().unwrap();
        let char_check: char = line_split[2].parse().unwrap();
        let password = line_split[4];

        let first_char = password.chars().nth(lower - 1).unwrap();
        let second_char = password.chars().nth(upper - lower).unwrap();
        // println!("first: {}, second: {}", first_char, second_char);
        if first_char == second_char {
            continue;
        }
        if char_check == first_char || char_check == second_char {
            pass_count += 1;
        }
    }

    println!("{} valid passwords", pass_count);
}
