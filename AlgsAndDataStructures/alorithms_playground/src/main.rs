// HeapSort
// 1,000,000 i32s regular cargo run 9.06 secs
// 1,000,000 i32s regular cargo run --release <1 sec

// BinarySearch
// avg time to run on 1,000,000 ints -> 0.074 secs
// (with --release) -> 0.010 secs

//mod HeapSort;
//use HeapSort::*;
use std::io::*;
mod BinarySearch;
use BinarySearch::*;
mod Utils;
use Utils::*;
mod MergeSort;
use MergeSort::*;
use std::time::SystemTime;

fn main() {

    println!("---------------------------");
    println!("| Welcome to the Benchtop |");
    println!("---------------------------");
    
    let mut test_array: Vec<i32> = vec![1,30,5,4,8,2,7,9,3,1];
    vec_print(&test_array);
    println!("sorted? {}", is_sorted(&test_array, |x,y| x<=y));
    merge_sort(&mut test_array);
    vec_print(&test_array);
    println!("sorted? {}", is_sorted(&test_array, |x,y| x<=y));
    println!("has 30? {}", binary_search(&test_array, 30));
    
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

