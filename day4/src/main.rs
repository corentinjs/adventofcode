use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_lines_from_file() -> Vec<String> {
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
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    return lines;
}

//Part1
//fn check_horizontal(lines: &Vec<String>) -> i32 {
//    let mut x_total: i32 = 0;
//    let length = lines.iter().map(|line| line.len()).min().unwrap();
//
//    for line in lines {
//       for pos in 0..(length - 3) {
//            let chars: Vec<char> = line.chars().skip(pos).take(4).collect();
//            if chars == vec!['X', 'M', 'A', 'S'] || chars == vec!['S', 'A', 'M', 'X'] {
//                x_total += 1;
//            }
//        }
//    }
//    x_total
//}
//
//fn check_vertical(lines: &Vec<String>) -> i32 {
//    let mut x_total: i32 = 0;
//    let length = lines.iter().map(|line| line.len()).min().unwrap();
//
//    for pos in 0..length {
//        for start_line in 0..(lines.len() - 3) {
//            let chars: Vec<char> = (start_line..start_line + 4)
//                .map(|line_index| lines[line_index].chars().nth(pos).unwrap())
//                .collect();
//            if chars == vec!['X', 'M', 'A', 'S'] || chars == vec!['S', 'A', 'M', 'X'] {
//                x_total += 1;
//            }
//        }
//    }
//    x_total
//}
//
//fn check_diagonal(lines: &Vec<String>) -> i32 {
//    let mut x_total: i32 = 0;
//    let length = lines.iter().map(|line| line.len()).min().unwrap();
//
//    for start_line in 0..(lines.len() - 3) {
//        for pos in 0..(length - 3) {
//            let mut chars: Vec<char> = Vec::new();
//            for offset in 0..4 {
//                if let Some(char) = lines[start_line + offset].chars().nth(pos + offset) {
//                    chars.push(char);
//                } else {
//                    break;
//                }
//
//                if chars == vec!['X', 'M', 'A', 'S'] || chars == vec!['S', 'A', 'M', 'X'] {
//                    x_total += 1;
//                }
//            }
//        }
//    }
//
//    for start_line in 0..(lines.len() - 3) {
//        for pos in 3..length {
//            let mut chars: Vec<char> = Vec::new();
//            for offset in 0..4 {
//                if let Some(char) = lines[start_line + offset].chars().nth(pos - offset) {
//                    chars.push(char);
//                } else {
//                    break;
//                }
//
//                if chars == vec!['X', 'M', 'A', 'S'] || chars == vec!['S', 'A', 'M', 'X'] {
//                    x_total += 1;
//                }
//            }
//        }
//    }
//    x_total
//}

//Part2
fn check_diagonal(lines: &Vec<String>) -> i32 {
    let mut x_total: i32 = 0;
    let length = lines.iter().map(|line| line.len()).min().unwrap();

    for start_line in 0..(lines.len() - 2) {
        for pos in 0..(length - 2) {
            let mut chars: Vec<char> = Vec::new();

            if let Some(char) = lines[start_line].chars().nth(pos) {
                chars.push(char);
            } else {
                break;
            }

            if let Some(char) = lines[start_line].chars().nth(pos + 2) {
                chars.push(char);
            } else {
                break;
            }

            if let Some(char) = lines[start_line + 1].chars().nth(pos + 1) {
                chars.push(char);
            } else {
                break;
            }

            if let Some(char) = lines[start_line + 2].chars().nth(pos) {
                chars.push(char);
            } else {
                break;
            }

            if let Some(char) = lines[start_line + 2].chars().nth(pos + 2) {
                chars.push(char);
            } else {
                break;
            }

            if chars == vec!['M', 'M', 'A', 'S', 'S']
                || chars == vec!['M', 'S', 'A', 'M', 'S']
                || chars == vec!['S', 'M', 'A', 'S', 'M']
                || chars == vec!['S', 'S', 'A', 'M', 'M']
            {
                x_total += 1;
            }
        }
    }
    x_total
}

fn main() {
    let lines = get_lines_from_file();
    let total = check_diagonal(&lines);

    println!("Total XMAS : {}", total);
}
