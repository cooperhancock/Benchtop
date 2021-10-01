// avg time to run on 1,000,000 ints -> 0.006 secs


#include <iostream>
#include <chrono>
using namespace std;

int binary_search(int arr[], int p, int q, int v){
    int m = (p+q)/2;
    if(arr[m] == v){
        return m;
    }else if(q-p == 0){
        return -1;
    }else if(v < arr[m]){
        return binary_search(arr, p, m, v);
    }else{
        return binary_search(arr, m+1, q, v);
    }
}

int main(){
    
    cout << "BinarySearch Test" << endl;
    auto timeStart = chrono::high_resolution_clock::now();
    
    const int size = 1000000;
    int* array = new int[size];
    
    for(int i=0; i<size; i++){
        array[i] = i;
    }

    cout << array[0] << " " << array[1] << " " << array[size-1] << endl;
    cout << "has " << size-1 << "? " << binary_search(array, 0, size-1, size-1) << endl;
    cout << "has " << size << "? " << binary_search(array, 0, size-1, size) << endl;
    auto timeEnd = chrono::high_resolution_clock::now();
    chrono::duration<double> diff = timeEnd-timeStart;
    cout << "time: " << diff.count() << endl;
}