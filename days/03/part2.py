from collections import defaultdict

def find_gears(M, i, j):
    gears = []
    for k in range(max(0, i-1), min(i+2, len(M))):
        for l in range(max(0, j-1), min(j+2, len(M[i]))):
            if M[k][l] == '*':
                gears.append((k, l))
    return gears

with open('inputs/day03.txt') as file:
    M = []
    for line in file:
        M.append(line.strip() + '.')

    gear_nums = defaultdict(list)
    for i in range(len(M)):
        num = ""
        gears = set()
        for j in range(len(M[i])):
            if M[i][j].isdigit():
                num += M[i][j]
                gears = gears.union(find_gears(M, i, j))
            elif len(num) > 0:
                for gear in gears:
                    gear_nums[gear].append(int(num))
                num = ""
                gears = set()

    total = 0
    for gear in gear_nums:
        if len(gear_nums[gear]) == 2:
            total += gear_nums[gear][0] * gear_nums[gear][1]
    print(total)
