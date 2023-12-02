import re

with open('input.txt', 'r') as file:
    lines = [line.strip() for line in file.readlines()]

p2 = "one|two|three|four|five|six|seven|eight|nine"
map = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
    "1": 1,
    "2": 2, 
    "3": 3, 
    "4": 4, 
    "5": 5,
    "6": 6,
    "7": 7,
    "8": 8,
    "9": 9
}
sum = sum([map[re.findall(f"(?=(\d|{p2}))", line)[0]] * 10 + map[re.findall(f"(?=(\d|{p2}))", line)[-1]] for line in lines])
print(sum)