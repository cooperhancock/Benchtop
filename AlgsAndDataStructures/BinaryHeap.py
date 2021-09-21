# Binary Heap Playground

# 1,000,000 items in 19.63 secs

def print_heap(heap):
    s = ''
    level = 1
    for i in range(1,heap_size(heap)+1):
        s += str(heap[i]) + ' '
        if i == ((2**level) - 1):
            s += '\n'
            level +=1
    print(s)

# finds index of left and right children or parent for heap-formatted list
def left(i):
    return 2*i
def right(i):
    return 2*i + 1
def parent(i):
    return i/2
def swap(list, i, j):
    temp = list[i]
    list[i] = list[j]
    list[j] = temp

def heap_size(heap):
    return heap[0] # heap stores size at index 0 of list

def dec_heap_size(heap):
    heap[0] -= 1

# move item to appropriate place in heap
def heapify(heap, i):
    max = heap[i]
    pos = i
    if left(i) <= heap_size(heap) and heap[left(i)] > max:
        max = heap[left(i)]
        pos = left(i)
    if right(i) <= heap_size(heap) and heap[right(i)] > max:
        max = heap[right(i)]
        pos = right(i)
    if not pos == i:
        swap(heap, i, pos)
        heapify(heap, pos)

# move item to appropriate place in heap
# def heapify(heap, i):
#     pos = i
#     while True:
#         max = heap[i]
#         if left(i) <= heap_size(heap) and heap[left(i)] > max:
#             max = heap[left(i)]
#             pos = left(i)
#         if right(i) <= heap_size(heap) and heap[right(i)] > max:
#             max = heap[right(i)]
#             pos = right(i)
#         if not pos == i:
#             swap(heap, i, pos)
#             i = pos
#         else:
#             break

# make list into heap format
def make_heap(list):
    list.insert(0, len(list)) # 1-index array and track size of heap in pos 0
    for k in range((heap_size(list)//2), 0, -1):
        heapify(list, k)

# remove max from heap
def extractMax(heap: list):
    if heap_size(heap) == 1: return heap[1]
    max = heap[1]
    heap[1] = heap[heap_size(heap)]
    heapify(heap, 1)
    return max

def HeapSort(list):
    make_heap(list)
    original_size = heap_size(list)
    for i in range(1, original_size+1):
        max = extractMax(list)
        list[heap_size(list)] = max
        dec_heap_size(list)
    list.pop(0)

list1 = []
for i in range(1000000000,0, -1):
    list1.append(i)
print(list1[0], list1[1], list1[len(list1)-1])
HeapSort(list1)
print(list1[0], list1[1], list1[len(list1)-1])
