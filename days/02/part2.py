
def power(game):
    """Computes the power of the cubes required for the game"""

    cubes = {
        'red': 0,
        'green': 0,
        'blue': 0
    }

    for round in game.split('; '):
        for ball in round.split(', '):
            n, color = ball.split(' ')
            n = int(n)

            cubes[color] = max(cubes[color], n)

    return cubes['red'] * cubes['green'] * cubes['blue']

limits = {
    'red': 12,
    'green': 13,
    'blue': 14
}

with open('inputs/day02.txt') as file:
    total = 0
    for i,line in enumerate(file):
        _, game = line.split(': ')
        total += power(game.strip())

    print(total)
