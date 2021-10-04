// HeapSort
// 1,000,000 i32s regular cargo run 9.06 secs
// 1,000,000 i32s regular cargo run --release <1 sec

// BinarySearch
// avg time to run on 1,000,000 ints -> 0.074 secs
// (with --release) -> 0.010 secs

#![allow(non_snake_case)]
#![allow(dead_code)]

//mod HeapSort;
//use HeapSort::*;
mod BinarySearch;
use BinarySearch::*;
mod Utils;
use Utils::*;
mod ArraySorting;
use ArraySorting::*;
mod Subsequences;
use Subsequences::*;

fn main() {

    println!("---------------------------");
    println!("| Welcome to the Benchtop |");
    println!("---------------------------");
    
    let vec = rand_vec(10, 30);
    vec_print(&vec);
    println!("lis {}", lis(&vec));
    println!("min deletions {}", min_deletions(&vec));

    // println!("test QuickSort");
    // test_sort(quick_sort, 10, 100);
    // println!("test MergeSort");
    // test_sort(merge_sort, 10, 100);
    // println!("test HeapSort");
    // test_sort(heap_sort, 10, 100);
    
    // let timeStart = SystemTime::now();
    // let mut big_array: Vec<i32> = Vec::with_capacity(1000000);
    // for i in 0..1000001 {
    //     big_array.push(i);
    // }
    // println!("{} {} {}", big_array[0], big_array[1], big_array[big_array.len()-1]);
    // println!("has 1000000? {}", binary_search(&big_array, 0, 1000000, 1000000));
    // println!("has 1000001? {}", binary_search(&big_array, 0, 1000000, 1000001));
    // let timeEnd = SystemTime::now();
    // let time = timeEnd.duration_since(timeStart).expect("whoopsie");
    // println!("time: {}", (time.as_micros() as f64) / 1000000.0 );
}

