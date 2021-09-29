// 1,000,000 i32s regular cargo run 9.06 secs
// 1,000,000 i32s regular cargo run --release <1 sec

mod HeapSort;
use HeapSort::*;

fn main() {
    let mut big_array: Vec<i32> = Vec::with_capacity(1000000000);
    for i in (0..1000000001).rev() {
        big_array.push(i);
    }
    println!("{} {} {}", big_array[0], big_array[1], big_array[big_array.len()-1]);
    heap_sort(&mut big_array);
    println!("{} {} {}", big_array[0], big_array[1], big_array[big_array.len()-1]);
}

