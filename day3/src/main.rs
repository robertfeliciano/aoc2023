use regex::{Error, Regex};
use std::{
    collections::HashMap,
    fs
};

fn get_schematic(input: &str) -> Vec<String> {
    let myreader = fs::read_to_string(input).unwrap();
    let trimmed_lines: Vec<String> = myreader
        .lines()
        .map(|line| line.trim().to_string())
        .collect();
    return trimmed_lines;
}

fn part1(schematic: Vec<String>) -> Result<u32, Error> {
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
        for row in line-1..line+2 { // need to check row above and below
            for col in start-1..end+2 { // need to check column before and after number for symbols
                if let Some(_) = sym_pos.get(&(row, col)) { 
                    // if there is a symbol directly above/below or diagonally adjacent to number
                    sum += *num;
                }
            }
        }
    }

    Ok(sum)
}

fn main() {
    let schematic = get_schematic("src/input.txt");
    let s = part1(schematic).unwrap();
    println!("{}", s);
}
