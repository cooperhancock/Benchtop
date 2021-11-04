
class Activity:
    def __init__(self, name, start, finish) -> None:
        self.name = name
        self.start = start 
        self.finish = finish
    def __repr__(self) -> str:
        return f'{self.name}: {self.start} to {self.finish}'

# maximizes number of activities scheduled
def activity_selector(S: list) -> list:
    # base case
    if len(S) <= 1:
        return S
    # find greedy choice
    g = min(S, key=lambda x: x.finish)
    #print(f'greedy: {g}')
    # collect rest of items that start after the greedy choice ends
    rest = []
    for a in S:
        if a.start >= g.finish: 
            rest.append(a)
    #print(f'rest: {rest}')
    A = activity_selector(rest)
    A.insert(0, g)
    return A

names = ['class' + str(i) for i in range(1,12)]
s = [1, 3, 0, 5, 3, 5, 6, 8, 8, 2, 12]
f = [4, 5, 6, 7, 9, 9, 10, 11, 12, 14, 16]
activities = []
for i in range(len(names)):
    activities.append(Activity(names[i], s[i], f[i]))

print(activities)
print(activity_selector(activities))