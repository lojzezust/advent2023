
def checkNeighbors(M, i, j):
    for k in range(max(0, i-1), min(i+2, len(M))):
        for l in range(max(0, j-1), min(j+2, len(M[i]))):
            if not M[k][l].isdigit() and M[k][l] != '.':
                return True
    return False

with open('inputs/day03.txt') as file:
    M = []
    for line in file:
        M.append(line.strip() + '.')

    total = 0
    for i in range(len(M)):
        num = ""
        valid = False
        for j in range(len(M[i])):
            if M[i][j].isdigit():
                num += M[i][j]
                if checkNeighbors(M, i, j):
                    valid = True
            elif len(num) > 0:
                if valid:
                    total += int(num)
                num = ""
                valid = False
    print(total)
