// avg time to run on 1,000,000 ints -> 0.017 secs

public class BinarySearch {
    public static void main(String[] args){
        long startTime = System.currentTimeMillis();
        final int size = 1000000;
        int[] big_array = new int[size];
        for(int i=0; i<size; i++){
            big_array[i] = i;
        }
        System.out.printf("%d %d %d\n", big_array[0], big_array[1], big_array[size-1]);
        System.out.printf("has %d? %d\n", size-1, binary_search(big_array, 0, size-1, size-1));
        System.out.printf("has %d? %d\n", size, binary_search(big_array, 0, size-1, size));
        System.out.println("time: " + ((System.currentTimeMillis()-startTime)/1000.0));
    }

    // finds v in arr sorted in monotonically increasing order
    // returns index or -1 if not in array
    public static int binary_search(int[] arr, int p, int q, int v) {
        int m = Math.floorDiv((p+q), 2);
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
}


