use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
struct position {
    value: u8,
    marked: bool
}

fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file);

    let mut lines:Vec<String> = reader.lines().map(|x| x.expect("Could not parse line")).collect();
    let callouts:Vec<u8> = lines[0].clone().split(",").map(|x| x.parse::<u8>().unwrap()).collect();
    lines.remove(0); // Remove drawn numbers
    lines.remove(0); // Remove first empty line

    // Parse input into grids
    let mut grids:Vec<Vec<Vec<position>>> = Vec::new();
    let mut grid:Vec<Vec<position>> = Vec::new();
    for line in lines {
        let line = line.trim();
        dbg!(line);
        if line == "" { // Is splitting two grids?
            grids.push(grid.clone());
            grid = Vec::new();
            continue
        };

        let row_raw = line.split(" ").collect::<Vec<&str>>();
        let mut row:Vec<position> = Vec::new();
        for element in row_raw { // Convert row elements into position objects
            if element == "" { // Means that the number is only one digit.
                continue;
            }
            row.push(
                position {
                    value: element.trim().parse().expect("Element not u8"),
                    marked: false,
                }
            );
        }
        grid.push(row.clone());
    }

    let result = play_game(grids.clone(), callouts.clone());
    let score = get_score(result.0, result.1);

    println!("{}", score);

    let result = play_game_to_end(grids, callouts);
    let score = get_score(result.0, result.1);
    println!("{}", score);
}

fn check_grid(grid: &Vec<Vec<position>>) -> bool { // Check if a grid has won
    // Check each row
    for row in grid.clone() {
        if check_row(&row) {
            return true;
        }
    }

    // Generate columns
    let mut columns:Vec<Vec<position>> = vec![Vec::new(); grid.len()];
    for row in grid.clone() {
        for (index, column) in row.iter().enumerate() {
            columns[index].push(column.clone());
        }
    }

    // Check each column
    for column in columns {
        if check_row(&column) {
            return true;
        }
    }

    return false;
}

fn check_row(row: &Vec<position>) -> bool { // Check if a row has all been marked
    for element in row {
        if element.marked == false {
            return false;
        }
    }

    return true;
}

fn mark_grid(grid: Vec<Vec<position>>, callout: u8) -> Vec<Vec<position>> { // Mark matching numbers in grid
    let mut new_grid = grid.clone();
    for mut row in  &mut new_grid {
        for mut element in row {
            if element.value == callout {
                element.marked = true;
            }
        }
    }


    return new_grid;
}

fn play_game(mut grids: Vec<Vec<Vec<position>>>, callouts: Vec<u8>) -> (Vec<Vec<position>>, u8) { // Play the game and return the winning grid
    for callout in callouts {
        for (index, grid) in grids.clone().iter().enumerate() {
            let grid = mark_grid(grid.clone(), callout);
            grids[index] = grid.clone();
            if check_grid(&grid) {
                return (grid, callout);
            }
        }
    }

    panic!("No winner found");
}

fn play_game_to_end(mut grids: Vec<Vec<Vec<position>>>, callouts: Vec<u8>) -> (Vec<Vec<position>>, u8) { // Play the game until one grid is left
    for callout in callouts.clone() {
        for (index, grid) in grids.clone().iter().enumerate() {
            let grid = mark_grid(grid.clone(), callout);
            grids[index] = grid.clone();
            if check_grid(&grid) {
                grids[index] = vec![vec![position {value:0, marked:false}]];
            }
        }
        grids.retain(|x| x.len() > 1);
        if grids.len() == 1 {
            return play_game(grids, callouts);
        }
    }

    panic!("Something went terribly wrong.");

}

fn get_score(grid: Vec<Vec<position>>, callout: u8) -> usize {
    let mut score:usize = 0;
    for row in grid {
        for element in row {
            if element.marked == false {
                score = score + element.value as usize;
            }
        }
    }

    return score * callout as usize;
}