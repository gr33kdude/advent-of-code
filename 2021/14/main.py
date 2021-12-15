#!/usr/bin/env python3

import operator
from collections import defaultdict
from functools import reduce
import copy
import sys
import re

class Cost:
    N = 0
    alphabet = ""
    cost_map = {}

    def __init__(self, cost):
        self.cost = cost

    def setup(alphabet):
        Cost.alphabet = alphabet
        alphabet = list(Cost.alphabet)
        alphabet.sort()
        Cost.N = len(alphabet)
        
        for (i, a) in enumerate(alphabet):
            Cost.cost_map[a] = i

    def cost(letters):
        '''
        for (i, a) in alphabet.sort().enumerate():
            self.map[a] = i
        '''
        cost = [0]*N
        for c in letters:
            cost[ Cost.cost_map[c] ] += 1

        return Cost(cost)

    def __add__(self, other):
        cost = [0]*N

        for i in range(N):
            cost[i] = self.cost[i] + other.cost[i]

        return Cost(cost)

    def __sub__(self, other):
        cost = [0]*N

        for i in range(N):
            cost[i] = self.cost[i] + other.cost[i]

        return Cost(cost)

    def __str__(self):
        s = "Cost("
        arr = []

        A = sorted(Cost.cost_map.items(), key = lambda kv: kv[1])
        for (k, v) in A:
            arr.append( f"{k} = {self.cost[v]}" )
        s += ", ".join(arr)
        s += ")"

        return s

class Bigram:
    def __init__(self, bigram, step):
        self.bigram = bigram
        self.step = step

class AltCost:
    def __init__(self, cost):
        self.cost = cost

    def cost(bigram):
        C = defaultdict(int)
        for c in bigram:
            C[c] += 1

        return Cost(C)

    def operate(self, other, func):
        new_cost = {}
        both_keys = set(self.cost.keys()) | set(other.cost.keys())
        for k in both_keys:
            new_cost[k] = func(self.cost.get(k, 0), other.cost.get(k, 0))

        return Cost(new_cost)

    def __add__(self, other):
        return self.operate(other, operator.add)

    def __sub__(self, other):
        return self.operate(other, operator.sub)

    def __str__(self):
        s = "Cost("
        arr = []
        for k in self.cost:
            arr.append( f"{k} = {self.cost[k]}" )
        s += ", ".join(arr)
        s += ")"

        return s

def cost(bigram, step, rules):
    global memo

    if step == 0:
        return Cost.cost(bigram)

    bg = Bigram(bigram, step)

    if bg not in memo:
        char = rules[bigram]

        left = cost(bigram[0] + char, step - 1, rules)
        right = cost(char + bigram[1], step - 1, rules)
        bigram_cost = left + right - Cost.cost(char)

        memo[bg] = bigram_cost

    return memo[bg]

def alt_cost(bigram, step, rules):
    global memo

    if step == 0:
        return Cost.cost("")

    new = rules[bigram]
    L_ = bigram[0] + new
    L_bg = Bigram(L_, step)

    R_ = new + bigram[1]
    R_bg = Bigram(R_, step)

    if L_bg not in memo:
        L = alt_cost(L_, step-1, rules) 
        memo[L_bg] = L

    if R_bg not in memo:
        R = alt_cost(R_, step-1, rules)
        memo[R_bg] = R

    return Cost.cost(new) + memo[L_bg] + memo[R_bg]

if len(sys.argv) != 2:
    print(f"Usage: {sys.argv[0]} <FILE>")
    sys.exit(1)

fname = sys.argv[1]
with open(fname, 'r') as f:
    lines = [line.rstrip('\n') for line in f.readlines()]

polymer = lines[0]

# build map
rules = {}
for line in lines[2:]:
    m = re.match("([A-Z][A-Z]) -> ([A-Z])", line)
    bigram = m.group(1)
    char = m.group(2)

    # print(f"BIGRAM = {bigram}, char = {char}")
    rules[bigram] = char

# find max number of letters
alphabet = set()
for k in rules.keys():
    for c in k:
        alphabet.add(c)

N = len(alphabet)
Cost.setup(alphabet)
#print(f"N = {N}")

generations = 40

memo = {}

polymer_bigrams = [polymer[idx:idx+2] for idx in range(len(polymer) - 1)]

cur_map = copy.deepcopy(rules)
for k in cur_map.keys():
    cur_map[k] = Cost.cost(k)

next_map = copy.deepcopy(rules)
for i in range(generations):
    for k, v in cur_map.items():
        c = rules[k]
        L = k[0]+c
        R = c+k[1]

        #print(f"k = {k}, v = {v}, {k} -> {c}, L = {L}, R = {R}")

        next_map[k] = cur_map[L] + cur_map[R]
    cur_map = copy.deepcopy(next_map)

bigram_costs = [ next_map[bigram] for bigram in polymer_bigrams ]
total_cost = reduce(lambda a, b: a+b, bigram_costs)

'''
bigram_costs = [ alt_cost(bigram, generations, rules) for bigram in polymer_bigrams ]
total_cost = Cost.cost(polymer)

for bgc in bigram_costs:
    total_cost += bgc
'''

#print("Cost of {},{} is: {}".format(polymer, generations, total_cost))

'''
print(f"{total_cost.cost}")
for v in sorted(total_cost.cost):
    print(f"{v}")
'''

max_cost = max(total_cost.cost)
min_cost = min(total_cost.cost)

print(f"The val: {(max_cost - min_cost)/2 + 1}")
