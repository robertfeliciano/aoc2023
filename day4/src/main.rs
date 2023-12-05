use std::{
    collections::HashSet,
    fs,
    io::Error,
};

fn get_cards(input: &str) -> Vec<(Vec<u8>, Vec<u8>)> {
    let myreader = fs::read_to_string(input).unwrap();
    let trimmed_lines: Vec<String> = myreader
        .lines()
        .map(|line| line.trim().to_string())
        .collect();

    let mut new_cards: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();

    for card in trimmed_lines {
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
    return new_cards;
}

fn part1(new_cards: &Vec<(Vec<u8>, Vec<u8>)>) -> Result<u32, Error> {
    let mut points: u32 = 0;
    let base: u32 = 2;

    for (winning_numbers, my_numbers) in new_cards {
        let mut numbers: HashSet<u8> = HashSet::new();
        for num in winning_numbers {
            numbers.insert(*num);
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

fn part2(cards: &Vec<(Vec<u8>, Vec<u8>)>) -> Result<u32, Error> {
    let mut line_no = 0;

    let mut counts: Vec<u32> = vec![1; cards.len()];

    for (winning_numbers, my_numbers) in cards {
        let mut numbers: HashSet<u8> = HashSet::new();
        for num in winning_numbers {
            numbers.insert(*num);
        }
        let mut same = 0;
        for num in my_numbers {
            if numbers.contains(&num) {
                same += 1;
            }
        }
        for c in (line_no + 1)..(line_no + same + 1) {
            counts[c] += 1 * counts[line_no];
        }
        line_no += 1;
    }

    let s = counts.iter().sum();

    Ok(s)
}

fn main() {
    let cards = get_cards("src/input.txt");
    let p1 = part1(&cards);
    println!("{}", p1.unwrap());
    let p2 = part2(&cards);
    println!("{}", p2.unwrap());
}
