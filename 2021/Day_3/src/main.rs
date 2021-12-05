use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut gamma = vec![""; 12];
    let mut epsilon = vec![""; 12];

    let mut gamma_sums = vec![0; 12];

    let reports:Vec<String> = reader.lines().map(|x| x.expect("Could not parse line")).collect();

    for line in reports.clone() {
        for (position, content) in line.chars().enumerate() {
            gamma_sums[position] = gamma_sums[position] + match content {
                '0' => -1,
                '1' => 1,
                _ => panic!("Invalid character")
            };
        }
    }

    for (index, _) in gamma.clone().iter().enumerate() {
        if gamma_sums[index] > 0 {
            gamma[index] = "1";
            epsilon[index] = "0";
        } else if gamma_sums[index] < 0 {
            gamma[index] = "0";
            epsilon[index] = "1";
        }
    }

    // Binary to decimal
    let gamma = usize::from_str_radix(&gamma.join(""), 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon.join(""), 2).unwrap();

    println!("Power Consumption: {}", gamma*epsilon);

    // Find oxygen rating
    let mut reports_oxygen = reports.clone();
    let mut position = 0;
    while reports_oxygen.len() > 1 {
        let mcb = most_common(&reports_oxygen, position, '1');
        reports_oxygen = filter(&reports_oxygen, position, mcb);
        position = position + 1;
    }

    // Find co2 scrubber rating
    let mut reports_co2 = reports.clone();
    let mut position = 0;
    while reports_co2.len() > 1 {
        let mcb = least_common(&reports_co2, position, '0');
        reports_co2 = filter(&reports_co2, position, mcb);
        position = position + 1;
    }

    let oxygen = usize::from_str_radix(&reports_oxygen[0], 2).unwrap();
    let co2 = usize::from_str_radix(&reports_co2[0], 2).unwrap();

    println!("Life Support Rating: {}", oxygen*co2);
}

fn most_common(readings: &Vec<String>, position: usize, default: char) -> char {
    let mut sum = 0;
    for line in readings.clone() {
        sum = sum + match line.chars().nth(position).unwrap() {
            '0' => -1,
            '1' => 1,
            _ => panic!("Invalid character")
        }
    }

    return if sum > 0 {
        '1'
    } else if sum < 0 {
        '0'
    } else {
        default
    }
}

fn least_common(readings: &Vec<String>, position: usize, default: char) -> char {
    let mut sum = 0;
    for line in readings.clone() {
        sum = sum + match line.chars().nth(position).unwrap() {
            '0' => -1,
            '1' => 1,
            _ => panic!("Invalid character")
        }
    }

    return if sum > 0 {
        '0'
    } else if sum < 0 {
        '1'
    } else {
        default
    }
}

fn filter(readings: &Vec<String>, position: usize, bit_value: char) -> Vec<String> {
    let mut output:Vec<String> = Vec::new();

    for line in readings.clone() {
        if line.chars().nth(position).unwrap() == bit_value {
            output.push(line.clone());
        }
    }

    return output;
}