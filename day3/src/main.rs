use regex::{Error, Regex};
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn get_schematic(input: &str) -> Vec<String> {
    let myreader = fs::read_to_string(input).unwrap();
    let trimmed_lines: Vec<String> = myreader
        .lines()
        .map(|line| line.trim().to_string())
        .collect();
    return trimmed_lines;
}

fn part2(schematic: &Vec<String>) -> Result<u32, Error> {
    let num = Regex::new(r"\d+")?;
    let sym = Regex::new(r"[^\d\.]")?;

    let mut num_pos: HashMap<(i16, i16), u32> = HashMap::new();

    let mut sym_pos: HashMap<(i16, i16), String> = HashMap::new();

    for (row, line) in schematic.iter().enumerate() {
        let numbers = num.find_iter(&line);
        let symbols = sym.find_iter(&line);

        for mat in numbers {
            let start = mat.start() as i16;
            let end = mat.end() as i16;
            let n = mat.as_str().parse::<u32>().unwrap();
            // need to store the number for its whole span so the dy/dx below works
            for col in start..end {
                num_pos.insert((row as i16, col), n);
            }
        }

        for mat in symbols {
            let col = mat.start() as i16;
            let s = mat.as_str().to_string();
            sym_pos.insert((row as i16, col), s);
        }
    }

    let mut sum: u32 = 0;

    for ((row, col), symbol) in sym_pos.iter() {
        if symbol != "*" {
            continue;
        }
        let mut adj_set: HashSet<&u32> = HashSet::new();
        let directions = vec![
            (0, 1),
            (1, 0),
            (0, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, 0),
            (0, -1),
            (-1, -1),
        ];
        for (dy, dx) in directions {
            if let Some(n) = num_pos.get(&(row + dy, col + dx)) {
                let _ = adj_set.insert(n);
            }
        }
        if adj_set.len() != 2 {
            continue;
        }
        sum += adj_set.iter().fold(1, |acc, &y| (acc * y) as u32);
    }

    Ok(sum)
}

fn part1(schematic: &Vec<String>) -> Result<u32, Error> {
    let num = Regex::new(r"\d+")?;
    let sym = Regex::new(r"[^\d\.]")?;

    let mut num_pos: HashMap<(i16, i16, i16), u32> = HashMap::new();

    let mut sym_pos: HashMap<(i16, i16), String> = HashMap::new();

    for (row, line) in schematic.iter().enumerate() {
        let numbers = num.find_iter(&line);
        let symbols = sym.find_iter(&line);

        for mat in numbers {
            let start = mat.start() as i16;
            let end = (mat.end() - 1) as i16;
            let n = mat.as_str().parse::<u32>().unwrap();
            num_pos.insert((row as i16, start, end), n);
        }

        for mat in symbols {
            let col = mat.start() as i16;
            let s = mat.as_str().to_string();
            sym_pos.insert((row as i16, col), s);
        }
    }

    let mut sum: u32 = 0;

    for ((line, start, end), num) in num_pos.iter() {
        for row in line - 1..line + 2 {
            // need to check row above and below
            for col in start - 1..end + 2 {
                // need to check column before and after number for symbols
                if let Some(_) = sym_pos.get(&(row, col)) {
                    // if there is a symbol directly above/below or diagonally adjacent to number
                    sum += *num;
                    break;
                }
            }
        }
    }

    Ok(sum)
}

fn main() {
    let schematic = get_schematic("src/input.txt");
    let s = part1(&schematic).unwrap();
    println!("{}", s);
    let s2 = part2(&schematic).unwrap();
    println!("{}", s2);
}
