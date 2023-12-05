
with open('inputs/day05.txt') as file:
    it = iter(file)
    seeds = [int(x) for x in next(it).strip().split(': ')[1].split(' ')]
    print(seeds)

    next(it)

    cur_lev = seeds
    for i in range(7):
        next(it)
        next_lev = [x for x in cur_lev]
        while (line := next(it, '').strip()) != '':
            sv, sk, r = (int(x) for x in line.split(' '))
            for i,val in enumerate(cur_lev):
                if val in range(sk, sk+r):
                    next_lev[i] = sv + val - sk

        cur_lev = next_lev

        # print(next_lev)

    print(min(next_lev))
