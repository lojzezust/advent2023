import re

nmap = {'one': 1, 'two': 2, 'three': 3,
        'four': 4, 'five': 5, 'six': 6,
        'seven': 7, 'eight': 8, 'nine': 9}

regex = r'one|two|three|four|five|six|seven|eight|nine|\d'
regex_rev = r'eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d'


total = 0
with open("days/01/input.txt") as file:
    for line in file:
        # Find first match (digit or digit-word) from both directions
        s1 = re.search(regex, line).group()
        s2 = re.search(regex_rev, line[::-1]).group()[::-1]

        # Convert digit-words to digits
        if not s1.isdigit():
            s1 = str(nmap[s1])
        if not s2.isdigit():
            s2 = str(nmap[s2])

        val = int(s1 + s2) # Combine first and last digit
        total += val

print("Part 2:", total)
