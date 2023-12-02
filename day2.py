from collections import defaultdict
import re

with open('input.txt', 'r') as file:
    lines = [line.strip() for line in file.readlines()]

RED=12
GREEN=13
BLUE=14

def get_games(lines):
    games = defaultdict(int)
    for line in lines:
        game_no = int(re.match(r'Game (\d+):', line).group(1))
        pulls = line.split(";")
        to_be_added = []
        for pull in pulls:
            r = re.search(r'(\d+) red', pull)
            g = re.search(r'(\d+) green', pull)
            b = re.search(r'(\d+) blue', pull)
            red = int(r.group(1)) if r else 0
            green = int(g.group(1)) if g else 0
            blue = int(b.group(1)) if b else 0
            to_be_added.append((red,green,blue))
        games[game_no] = to_be_added
    return games

def part1(lines):
    games = get_games(lines)

    possible = [0]*101

    for idx, game in games.items():
        game_not_possible = False
        for r,g,b in game:
            if r <= RED and g <= GREEN and b <= BLUE:
                continue
            else:
                game_not_possible = True
                break
        if game_not_possible:
            continue
        else:
            possible[idx] = idx
    print(sum(possible))

def part2(lines):
    games = get_games(lines)

    sum_power_set = [0]*101
    for idx, game in games.items():
        get_color = lambda idx, pulls : list(map(lambda e: e[idx], pulls ))
        red_needed = max(get_color(0, game))
        green_needed = max(get_color(1, game))
        blue_needed = max(get_color(2, game))
        sum_power_set[idx] = red_needed * green_needed * blue_needed


    print(sum(sum_power_set))





part2(lines)