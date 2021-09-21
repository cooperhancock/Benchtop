package AlgsAndDataStructures;

public class JavaBinaryHeap{
    public static void main(String[] args){
        int[] list1 = {1,2,3,4,5};
        print_list(list1);
        make_heap(list1);
        print_list(list1);
    }

    public static void print_list(int[] list){
        for(int i=0; i<list.length; i++){
            System.out.print(i);
            System.out.print(" ");
        }
        System.out.println();
    }

    public static void make_heap(int[] list){
        list[0] = 100;
    }
}