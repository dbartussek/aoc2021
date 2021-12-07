with open("input/07.txt") as f:
	crabs = [int(s) for s in f.read().strip().split(',')]

print(min([sum([abs(pos - destination) for pos in crabs]) for destination in range(min(crabs), max(crabs) + 1)]))
print(min([sum([sum(range(abs(pos - destination) + 1)) for pos in crabs]) for destination in range(min(crabs), max(crabs) + 1)]))
