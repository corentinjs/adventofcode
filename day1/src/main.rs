use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_lines_from_file() -> std::io::Lines<BufReader<File>> {
    let path = Path::new("./src/input.txt");
    let display = path.display();
    let file = match File::open(&path) {
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

fn number_from_lines(lines: std::io::Lines<BufReader<File>>) -> (Vec<i32>, Vec<i32>) {
    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();

    for l in lines {
        match l {
            Ok(l) => {
                let collection: Vec<&str> = l.split_whitespace().collect();
                match (collection[0].parse::<i32>(), collection[1].parse::<i32>()) {
                    (Ok(a), Ok(b)) => {
                        list_a.push(a);
                        list_b.push(b);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    list_a.sort();
    list_b.sort();
    return (list_a, list_b);
}

fn main() {
    let lines: std::io::Lines<BufReader<File>> = get_lines_from_file();
    let (list_a, list_b) = number_from_lines(lines);

    let mut total: i32 = 0;

    // DAY1 PART 1
    //for (a, b) in list_a.iter().zip(list_b.iter()) {
    //    let calc = *a - *b;
    //    total += calc.abs();
    //}

    //DAY1 PART 2
    for a in list_a.iter() {
        let mut count: i32 = 0;
        for b in list_b.iter() {
            if *a == *b {
                count += 1;
            }
        }
        let calc = *a * count;
        total += calc;
    }

    println!("Total : {}", total);
}
