import os
from enum import Enum

class Modes(Enum):
    PART1 = 1,
    PART2 = 2
    
MODE = Modes.PART1

with open("day18/input.txt", "r") as file:
    commands = [line.strip().split(" ") for line in file.readlines()]
    for command in commands:
        command[1] = int(command[1])
        command[2] = command[2][2:-1]

if MODE == Modes.PART2:
    num_to_dir = {
        "0": "R",
        "1": "D",
        "2": "L",
        "3": "U"    
    }
    
    for command in commands:
        color = command[2]
        command[1], command[0] = (int(color[:-1], base=16), num_to_dir[color[-1]])
        

directions = {
    "L": (0,-1),
    "R": (0,1),
    "D": (1,0),
    "U": (-1,0)
}

# determine the size of the board
pointer = (0,0)
maxes = (0,0)
mins = (0,0)
for (dir, length, _) in commands:
    change = directions[dir]
    for i in range(length):
        pointer = tuple(sum(x) for x in zip(pointer, change))
    maxes = tuple(max(x) for x in zip(pointer, maxes))
    mins = tuple(min(x) for x in zip(pointer, mins))
        
map = [ [0] * (maxes[1] - mins[1] + 1) for _ in range(maxes[0] - mins[0] + 1)]
def print_map():
    print("\nmap:")
    for row in map:
        print(" ".join(["#" if x==1 else " " for x in row]))
        
# create edge on map
start = (0 - mins[0],0 - mins[1])
pointer = start
# map[0][0] = 1
for (dir, length, _) in commands:
    change = directions[dir]
    for i in range(length):
        pointer = tuple(sum(x) for x in zip(pointer, change))
        map[pointer[0]][pointer[1]] = 1

print_map()

# fill map, using dfs
queue = [(0 - mins[0] + 1,0 - mins[1] + 1)]
while len(queue) > 0:
    row, col = queue.pop()
    if map[row][col] == 1:
        continue
    map[row][col] = 1
    if row < len(map) - 1:
        queue.append((row+1, col))
    if row > 0:
        queue.append((row-1, col))
    if col > 0:
        queue.append((row, col-1))
    if col < len(map[row]) -1:
        queue.append((row, col+1))

        
print_map()

count = 0
for row in map:
    count += sum(row)

print(f"\nBLOCKS {count}")

