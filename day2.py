from functools import reduce
import re

with open('input.txt', 'r') as file:
    lines = [line.strip() for line in file.readlines()]

RED=12; GREEN=13; BLUE=14

def get_games(lines):
    games = {}

    for line in lines:
        match = re.match(r'Game (\d+):(.+)', line)
        if match:
            game_no, pulls = int(match.group(1)), match.group(2).split(';')
            games[game_no] = [(int(r) if r else 0, int(g) if g else 0, int(b) if b else 0) for pull in pulls for r,g,b in re.findall(r'(\d+) red|(\d+) green|(\d+) blue', pull)]
    return games

def part1(lines):
    games = get_games(lines)

    possible = [idx for idx, game in games.items() if not any([r > RED or g > GREEN or b > BLUE for r, g, b in game])]
    
    print(sum(possible))

def part2(lines):
    games = get_games(lines)

    get_color = lambda idx, pulls : list(map(lambda e: e[idx], pulls ))
    multiply = lambda lst: reduce(lambda x, y: x*y, lst)
    power_set = [multiply([max(get_color(c, game)) for c in range(0,3)]) for _, game in games.items()]

    print(sum(power_set))

part1(lines)
part2(lines)