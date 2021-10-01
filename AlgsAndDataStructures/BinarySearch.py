import time

# takes array sorted in increasing order, beginning and end indicies, and value to find
# returns index of value or 
def binary_search(arr, p, q, v):
    m = (p+q)//2
    if arr[m] == v:
        return m
    elif q-p == 0:
        return -1
    elif v < arr[m]:
        return binary_search(arr, p, m, v)
    else:
        return binary_search(arr, m+1, q, v)

timeStart = time.time()
array = []
for i in range(1000001):
    array.append(i)

print(array[0], array[1], array[len(array)-1])
print('has 1000000?', binary_search(array, 0, 1000000, 1000000))
print('has 1000001?', binary_search(array, 0, 1000000, 1000001))
print('time', time.time() - timeStart)
