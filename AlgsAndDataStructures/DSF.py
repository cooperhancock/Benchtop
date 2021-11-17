class dsf:
    def __init__(self, set: set) -> None:
        self.p = {i: None for i in set} # initialize parent of each item to None (not in any set)
        self.rank = {i: None for i in set} # initialize each rank to None (not in any set)
    
    def __repr__(self) -> str:
        sets = {i: [] for i in list(self.p.values())}
        for i in self.p:
            sets[self.p[i]].append(i)
        s = ''
        for i in sets:
            if not i == None: 
                s += '{'
                for j in sets[i]:
                    s += f'{j}, '
                s = s[:len(s)-2] + '} '
        return s

    # makes set in dsf
    def make_set(self, x) -> None:
        try:
            self.p[x] = x
            self.rank[x] = 0
        except KeyError:
            print('Error: indicated value not in dataset')

    # finds representative node of a set (i.e. the set the node is in)
    def find_set(self, x):
        try:
            if not x == self.p[x]:
                self.p[x] = self.find_set(self.p[x])
            return self.p[x]
        except KeyError:
            print('Error: indicated value not in dataset')

    # join 2 sets together
    def union(self, x, y) -> None:
        try:
            i = self.find_set(x)
            j = self.find_set(y)
            if self.rank[i] > self.rank[j]:
                self.p [j] = i
            else:
                self.p[i] = j
                if self.rank[i] == self.rank[j]:
                    self.rank[j] += 1
        except KeyError:
            print('Error: indicated value not in dataset')

if __name__ == "__main__":
    s = {0, 1, 2, 3, 4, 6, 7}
    ds = dsf(s)
    print(ds)
    for i in s:
        ds.make_set(i)
    ds.union(7, 0)
    print(ds)
    ds.union(1, 3)
    ds.union(4, 6)
    ds.union(2, 6)
    print(ds)
    print(ds.rank)