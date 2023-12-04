use std::{collections::HashSet, fs, io::Error};

fn get_cards(input: &str) -> Vec<String> {
    let myreader = fs::read_to_string(input).unwrap();
    let trimmed_lines: Vec<String> = myreader
        .lines()
        .map(|line| line.trim().to_string())
        .collect();
    return trimmed_lines;
}

fn part1(cards: &Vec<String>) -> Result<u32, Error> {
    let mut new_cards = Vec::new();

    for card in cards {
        let new_card: &mut Vec<_> = &mut card[9..].split("|").collect();
        let winning: Vec<u8> = new_card[0]
            .to_string()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        let mine: Vec<u8> = new_card[1]
            .to_string()
            .split_ascii_whitespace()
            .map(|s| s.parse::<u8>().unwrap())
            .collect();
        new_cards.push((winning, mine));
    }

    let mut points: u32 = 0;
    let base: u32 = 2;

    for (winning_numbers, my_numbers) in new_cards {
        let mut numbers: HashSet<u8> = HashSet::new();
        for num in winning_numbers {
            numbers.insert(num);
        }
        let mut same = 0;
        for num in my_numbers {
            if !numbers.contains(&num) {
                continue;
            } else {
                same += 1
            }
        }
        if same > 0 {
            points += base.pow(same - 1)
        }
    }
    Ok(points)
}

fn part2(cards: &Vec<String>) -> Result<u16, Error> {

    
    Ok(2)
}

fn main() {
    let cards = get_cards("src/input.txt");
    let p1 = part1(&cards);
    println!("{}", p1.unwrap());
    part2(&cards);
}
