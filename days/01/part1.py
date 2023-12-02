
total = 0
with open("inputs/day01.txt") as file:
    for line in file:
        digits = [c for c in line if c.isdigit()]
        val = int(digits[0] + digits[-1]) # Combine first and last digit
        total += val

print("Part 1:", total)
