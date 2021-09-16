#include <iostream>
using namespace std;

int main(){
    const int arr_size = 1000000000;
    int arr[arr_size];
    for (int i=0; i<arr_size; i++){
        arr[i] = i;
    }
    cout << "done" << endl;
}