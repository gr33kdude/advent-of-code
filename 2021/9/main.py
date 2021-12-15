#!/usr/bin/env python3

import sys
from functools import reduce
import operator

def valid(row, rows, col, cols):
    return row in range(rows) and col in range(cols)

def print_basins(basins):
    rows = len(basins)
    cols = len(basins[0])

    sys.stdout.write(' ')
    for i in range(cols):
        sys.stdout.write('_')
    sys.stdout.write(' ')
    sys.stdout.write('\n')

    for (r, row) in enumerate(basins):
        sys.stdout.write('|')
        for (c, col) in enumerate(row):
            #sys.stderr.write(f" --> basins[{r}][{c}] = {basins[r][c]}\n")
            if basins[r][c] < 9:
                sys.stdout.write(' ')
            elif basins[r][c] == 9:
                sys.stdout.write('*')
            elif basins[r][c] > 9:
                sys.stdout.write(chr(ord('A') + (basins[r][c] - 10)))

        sys.stdout.write('|')
        sys.stdout.write("\n")

    sys.stdout.write(' ')
    for i in range(cols):
        sys.stdout.write('^')
    sys.stdout.write(' ')
    sys.stdout.write('\n')

    sys.stdout.flush()

if len(sys.argv) != 2:
    print(f"Usage: {sys.argv[0]} <FILE>")
    sys.exit(1)

fname = sys.argv[1]
with open(fname, 'r') as f:
    lines = [line.rstrip() for line in f.readlines()]

matrix = [[ int(c) for c in line ] for line in lines]

rows = len(matrix)
cols = len(matrix[0])

least = []
for (r, row) in enumerate(matrix):
    for (c, col) in enumerate(row):
        all_locs = [ (r, c-1), (r, c+1), (r-1, c), (r+1, c) ]
        check_locs = filter(lambda tup: valid(tup[0], rows, tup[1], cols), all_locs)

        current = matrix[r][c]
        satisfied = [current < matrix[i][j] for (i, j) in check_locs]
        if all(satisfied):
            least.append(current + 1)

print( sum(least) )


basins = matrix[:]
num_basins = []
basin_count = 0
for r in range(rows):
    for c in range(cols):
        if basins[r][c] == 9:
            continue
        if r-1 not in range(rows):
            up = -1
        else:
            up = basins[r-1][c]

        if c-1 not in range(cols):
            left = -1
        else:
            left = basins[r][c-1]

        if up > 9 or left > 9:
            '''
            if not (up == left or up in [-1, 9] or left in [-1, 9]):
                print("===== ERROR!! =====")
                print(f"at [{r}][{c}] up: {up}, left: {left}")
                print_basins(basins)
                #assert(up == left or up == 9 or left == 9)
                sys.exit(1)
            '''
            basins[r][c] = up if up > 9 else left
            basin = basins[r][c] - 10
            num_basins[basin] += 1
        else:
            #assert(up in [-1, 9] and left in [-1, 9])
            basins[r][c] = (basin_count + 10) % 36
            num_basins.append(1)
            basin_count += 1

    '''
    for c in range(cols-1, 0, -1):
        cur = basins[r][c]
        left = basins[r][c-1]

        if cur == 9 or left == 9:
            continue

        if left != cur:
            num_basins[left-10] -= 1
            basins[r][c-1] = cur
            num_basins[cur-10] += 1
    '''

print_basins(basins)

print(f"basin count: {basin_count}")
print(f"{num_basins}")

L = sorted(num_basins, reverse = True)[:3]
print(f"{L}")
ans = L[0] * L[1] * L[2]
print(f"{ans}")

