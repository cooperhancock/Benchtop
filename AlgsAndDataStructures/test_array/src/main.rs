// 1,000,000 i32s regular cargo run 9.06 secs
// 1,000,000 i32s regular cargo run --release <1 sec

//mod HeapSort;
//use HeapSort::*;
use std::io::*;
mod BinarySearch;
use BinarySearch::*;
use std::time::SystemTime;

fn main() {

    // println!("---------------------------");
    // println!("| Welcome to the Benchtop |");
    // println!("---------------------------");
    
    let timeStart = SystemTime::now();
    let mut big_array: Vec<i32> = Vec::with_capacity(1000000);
    for i in 0..1000001 {
        big_array.push(i);
    }
    println!("{} {} {}", big_array[0], big_array[1], big_array[big_array.len()-1]);
    println!("has 1000000? {}", binary_search(&big_array, 0, 1000000, 1000000));
    println!("has 1000001? {}", binary_search(&big_array, 0, 1000000, 1000001));
    let timeEnd = SystemTime::now();
    let time = timeEnd.duration_since(timeStart).expect("whoopsie");
    println!("time: {}", (time.as_micros() as f64) / 1000000.0 );
}

