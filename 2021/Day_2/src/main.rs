use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut horizontal:usize = 0;
    let mut depth:usize = 0;

    println!("Please select which part to answer. (1/2)");
    let mut part = String::new();
    std::io::stdin()
        .read_line(&mut part)
        .expect("Failed to read line");

    if part.trim() == "1" {
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line.contains("forward") {
                let line: usize = line.replace("forward ", "").parse().unwrap();
                horizontal = horizontal + line;
            } else if line.contains("down") {
                let line: usize = line.replace("down ", "").parse().unwrap();
                depth = depth + line;
            } else if line.contains("up") {
                let line: usize = line.replace("up ", "").parse().unwrap();
                depth = depth - line;
            }
        }
    } else if part.trim() == "2" {
        let mut aim = 0;
        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line.contains("forward") {
                let line: usize = line.replace("forward ", "").parse().unwrap();
                horizontal = horizontal + line;
                depth = depth + line*aim;
            } else if line.contains("down") {
                let line: usize = line.replace("down ", "").parse().unwrap();
                aim = aim + line;
            } else if line.contains("up") {
                let line: usize = line.replace("up ", "").parse().unwrap();
                aim = aim - line;
            }
        }
    } else {
        println!("Invalid choice");
    }

    println!("{}", depth*horizontal);
}
