# implementation of TSP
# problem represented as 2d matrix with keys being cities, and values being distances between cities
# example: 
# d = {'a': {'a': 0, 'b': 5, 'c': 7}, 'b': {'a': 5, 'b': 0, 'c': 10}, 'c': {'a': 7, 'b': 10, 'c': 0}}

import csv

raw_problem = []

# assemble raw problem from csv
with open('fewer_european_cities.csv') as csvfile:
    reader = csv.reader(csvfile)
    for row in reader:
        raw_problem.append(row)

# assemble dictionary from raw problem
def format_problem(r: list) -> dict:
    p = {}
    for i in range(1,len(r)):
        p[r[0][i]] = {}
        for j in range(1,len(r[i])):
            p[r[0][i]][r[0][j]] = int(r[i][j])
    return p

# calculates weight of a traversal across a list of nodes
def distance(p: dict, l: list, d):
    if len(l) < 2:
        return d
    return distance(p, l[1:], d + p[l[0]][l[1]])

# produces list of permutaitons of a list
def permutations(l: list, accum: list, r: list):
    if len(l) <= 1:
        r.append(accum + l)
        return r
    for i in range(len(l)):
        x = permutations(l[:i] + l[i+1:], accum + [l[i]], r)
    return r

# print formatted problem dictionary
def print_problem(p: dict) -> None:
    for key in p:
        print(f'{key}: {p[key]}')

# solves TSP using brute force
def factorial_tsp(p: dict) -> tuple:
    routes = permutations(list(p.keys()), [], [])
    for r in routes:
        r += [r[0]]
    costs = [distance(p, i, 0) for i in routes]
    return routes[costs.index(min(costs))], min(costs)

problem = format_problem(raw_problem)
print_problem(problem)
print(f'TSP brute force solution: {factorial_tsp(problem)}')