use std::{fs, process};

fn main() {
    let file_name = "b";
    let file = fs::read_to_string(format!("{}.in", file_name)).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        process::exit(1);
    });

    let contents = file.lines();
    let mut pass_count = 0;

    for line in contents {
        let line_split: Vec<&str> = line.split(|s| s == '-' || s == ' ' || s == ':').collect();
        // println!("{:?}", line_split);

        let lower: u32 = line_split[0].parse().unwrap();
        let upper: u32 = line_split[1].parse().unwrap();
        let char_check: char = line_split[2].parse().unwrap();
        let password = line_split[4];
        let mut count = 0;
        for char in password.chars() {
            if char == char_check {
                count += 1;
            }
        }
        if count >= lower && count <= upper {
            pass_count += 1;
        }
    }

    println!("{} valid passwords", pass_count);
}
