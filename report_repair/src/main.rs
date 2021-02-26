use std::{fs, process};

fn main() {
    let file_name = "b";
    let file = fs::read_to_string(format!("{}.in", file_name)).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {}", err);
        process::exit(1);
    });

    let contents = file.lines();
    let values: Vec<u32> = contents.map(|line| line.parse().unwrap()).collect();

    for i in &values {
        for j in &values {
            for k in &values {
                if (i + j + k) == 2020 {
                    fs::write(format!("{}.out", file_name), format!("{}", i * j * k))
                        .unwrap_or_else(|err| {
                            eprintln!("Problem writing file: {}", err);
                            process::exit(1);
                        });
                }
            }
        }
    }
}
