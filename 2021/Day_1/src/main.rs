use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;
use std::io;

fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);

    println!("How long should the sliding window be?\n");
    let mut window_size = String::new();
    io::stdin()
        .read_line(&mut window_size)
        .expect("Failed to read line.");
    let window_size:usize = window_size.trim().parse().expect("Input was not an int!");

    let mut buffer = VecDeque::new();

    let mut total:usize = 0;
    for (index, line) in reader.lines().enumerate() {
        if index < window_size {
            continue;
        }
        let previous_sum:usize = buffer.iter().sum();
        if buffer.len() > window_size-1 {
            buffer.pop_front();
        }
        let line:usize = line.unwrap().parse().unwrap();
        buffer.push_back(line);
        if buffer.iter().sum::<usize>() > previous_sum {
            total = total + 1;
        }
    }

    println!("{}", total);
}
