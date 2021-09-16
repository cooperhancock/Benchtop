# Binary Heap Playground

class BinaryHeap:
    def __init__(self, list=[]):
        self.heap = [None]
        size = 0
        for i in list:
            self.heap.append(i)
            size+=1
        for k in range((size//2), 0, -1):
            self.heapify(k)
    def __repr__(self):
        s = ''
        level = 1
        for i in range(1,self.size()+1):
            s += str(self.heap[i]) + ' '
            if i == ((2**level) - 1):
                s += '\n'
                level +=1
        return s
    def size(self):
        return len(self.heap)-1
    def left(self, i):
        return 2*i
    def right(self, i):
        return 2*i + 1
    def parent(self, i):
        return i/2
    def swap(self, i, j):
        temp = self.heap[i]
        self.heap[i] = self.heap[j]
        self.heap[j] = temp
    def heapify(self, i):
        max = self.heap[i]
        pos = i
        if self.left(i) <= self.size() and self.heap[self.left(i)] > max:
            max = self.heap[self.left(i)]
            pos = self.left(i)
        if self.right(i) <= self.size() and self.heap[self.right(i)] > max:
            max = self.heap[self.right(i)]
            pos = self.right(i)
        if not pos == i:
            self.swap(i, pos)
            self.heapify(pos)
    def extractMax(self):
        if self.size() == 1: return self.heap[1]
        max = self.heap[1]
        self.heap[1] = self.heap.pop()
        self.heapify(1)
        return max

def HeapSort(list):
    sort_heap = BinaryHeap(list)
    for i in range(len(list)):
        list[i] = sort_heap.extractMax()

list1 = []
for i in range(1000001):
    list1.append(i)
print('list made')
print(list1[0], list1[len(list1)//2], list1[len(list1)-1])
HeapSort(list1)
print(list1[0], list1[len(list1)//2], list1[len(list1)-1])
