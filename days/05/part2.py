from collections import deque

def intersection(c,k):
    s = max(c.start, k.start)
    e = min(c.stop, k.stop)
    if e <= s:
        return None, []

    remainders = []
    if s > c.start:
        remainders.append(range(c.start, s))

    if e < c.stop:
        remainders.append(range(e, c.stop))

    return range(s, e), remainders


with open('inputs/day05.txt') as file:
    it = iter(file)
    seeds = [int(x) for x in next(it).strip().split(': ')[1].split(' ')]
    seed_ranges = []
    for i in range(0, len(seeds), 2):
        seed_ranges.append(range(seeds[i], seeds[i] + seeds[i+1]))

    next(it)

    cur_lev = deque(seed_ranges)
    for i in range(7):
        next(it)
        next_lev = deque()

        # Read range map
        lev_map = {}
        while (line := next(it, '').strip()) != '':
            sv, sk, r = (int(x) for x in line.split(' '))
            lev_map[range(sk, sk+r)] = sv - sk

        # Map each of the ranges in cur_lev to the new ranges
        while cur_lev:
            c = cur_lev.popleft()
            found = False
            for k in lev_map:
                inter, remain = intersection(c,k)
                if inter is not None:
                    s = lev_map[k]
                    newr = range(inter.start + s, inter.stop + s)
                    next_lev.append(newr)

                    # Intersection may cut the range into multiple pieces, add them back to the queue
                    for r in remain:
                        cur_lev.append(r)

                    found = True
                    break

            if not found:
                next_lev.append(c)

        cur_lev = next_lev

    print(min(n.start for n in next_lev))
