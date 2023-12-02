
def is_valid(game):
    """Checks the validity of the game"""

    for round in game.split('; '):
        for ball in round.split(', '):
            n, color = ball.split(' ')
            n = int(n)

            # If any ball count exceeds the limit, the game is invalid
            if n > limits[color]:
                return False

    return True

limits = {
    'red': 12,
    'green': 13,
    'blue': 14
}

with open('inputs/day02.txt') as file:
    total = 0
    for i,line in enumerate(file):
        _, game = line.split(': ')
        if is_valid(game.strip()):
            total += i+1

    print(total)
