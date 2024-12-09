use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    static ref D: Regex = Regex::new(r"do\(\)").unwrap();
    static ref DT: Regex = Regex::new(r"don't\(\)").unwrap();
    static ref M: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
}

fn get_lines_from_file() -> std::io::Lines<BufReader<File>> {
    let path = Path::new("./src/input.txt");
    let file = File::open(path).unwrap();
    let reader: BufReader<File> = BufReader::new(file);
    let lines = reader.lines();
    return lines;
}

fn get_mut_and_calc_in_line(line: &str, mut e: bool) -> (i32, bool) {
    let mut total = 0;

    for match_str in RE.find_iter(line) {
        let match_str = match_str.as_str();
        if is_enable(match_str) {
            e = true;
            continue;
        } else if is_disable(match_str) {
            e = false;
            continue;
        } else if is_mul(match_str) {
            if e {
                if let Some(match_mul) = M.captures(match_str) {
                    let first_number = match_mul.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let second_number = match_mul.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    let calc = first_number * second_number;
                    total += calc;
                }
            }
        }
    }

    return (total, e);
}

fn is_mul(s: &str) -> bool {
    M.is_match(s)
}

fn is_enable(s: &str) -> bool {
    D.is_match(s)
}

fn is_disable(s: &str) -> bool {
    DT.is_match(s)
}

fn main() {
    let lines = get_lines_from_file();
    let mut results = 0;
    let mut enable: bool = true;

    for line in lines {
        let line = line.unwrap();
        let (res, n) = get_mut_and_calc_in_line(&line, enable);
        enable = n;
        results += res;
    }

    println!("results : {}", results);
}
