use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_lines_from_file() -> std::io::Lines<BufReader<File>> {
    let path = Path::new("./src/input.txt");
    let display = path.display();
    let file: File = match File::open(path) {
        Err(why) => panic!(
            "couldn't open {}: {}",
            display,
            <dyn Error>::to_string(&why)
        ),
        Ok(file) => file,
    };
    let reader: BufReader<File> = BufReader::new(file);
    let lines = reader.lines();
    return lines;
}

fn safe_unsafe(lines: std::io::Lines<BufReader<File>>) -> i32 {
    let mut qty_safe: i32 = 0;
    for l in lines {
        match l {
            Ok(l) => {
                let collection: Vec<i32> = l
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().expect("Erreur de parsing"))
                    .collect();

                if is_safed(&collection) || is_become_safe(&collection) {
                    qty_safe += 1;
                }
            }
            _ => {}
        }
    }
    return qty_safe;
}

fn is_safed(collection: &Vec<i32>) -> bool {
    let mut is_increasing = None;
    for i in 1..collection.len() {
        let prev_digit = collection[i - 1];
        let curr_digit = collection[i];
        let diff = (curr_digit - prev_digit).abs();

        if diff < 1 || diff > 3 {
            return false;
        }

        if is_increasing.is_none() {
            if curr_digit > prev_digit {
                is_increasing = Some(true);
            } else if curr_digit < prev_digit {
                is_increasing = Some(false);
            } else {
                return false;
            }
        }

        if let Some(increasing) = is_increasing {
            if (increasing && curr_digit <= prev_digit) || (!increasing && curr_digit >= prev_digit)
            {
                return false;
            }
        }
    }
    return true;
}

fn is_become_safe(collection: &Vec<i32>) -> bool {
    for (i, _n) in collection.iter().enumerate() {
        let mut modify = collection.clone();
        modify.remove(i);
        if is_safed(&modify) {
            return true;
        }
    }
    return false;
}

fn main() {
    let lines: std::io::Lines<BufReader<File>> = get_lines_from_file();
    let nbr_of_safe: i32 = safe_unsafe(lines);
    println!("nbr of safe : {}", nbr_of_safe);
}
