//                                             //
// UTILITY FUNCTIONS FOR ALGORITHMS PLAYGROUND //
//                                             //

use rand::prelude::*;

// returns true if comparitively sorted by the passed comparison function
pub fn is_sorted(arr: &Vec<i32>, f: fn(i32, i32) -> bool) -> bool{
    for i in 0..(arr.len()-1){
        if f(arr[i], arr[i+1]) != true{
            println!("{} and {} not sorted", arr[i], arr[i+1]);
            return false;
        }
    }
    return true;
}

// prints 1st, 2nd, and last items of Vec
pub fn quick_print(arr: &Vec<i32>){
    if arr.len() < 3 {
        for i in 0..(arr.len()){
            print!("{} ", arr[i]);
        }
        println!("");
    } else{
        println!("{}, {}, {}", arr[0], arr[1], arr[arr.len()-1]);
    }
}

// prints items in vec
pub fn vec_print(arr: &Vec<i32>){
    for i in 0..(arr.len()){
        print!("{} ", arr[i]);
    }
    println!("");
}

// generates vec of rand i32s from 0 to range of given size
pub fn rand_vec(size: usize, range: i32) -> Vec<i32>{
    let mut rng = thread_rng();
    let mut vec: Vec<i32> = Vec::with_capacity(size);
    for i in 0..(size){
        vec.push(rng.gen_range(0..range));
    } 
    return vec;
}

// tests sorting algorithm, runs iterations # of tests, with vec up to size limit
pub fn test_sort(algorithm: fn(&mut Vec<i32>), iterations: usize, limit: i32){
    let mut rng = thread_rng();
    for i in 0..iterations{
        let mut vec = rand_vec(rng.gen_range(5..limit) as usize, 4*limit);
        algorithm(&mut vec);
        if is_sorted(&vec, |x,y| x<=y) == false {
            println!("failed test on: ");
            vec_print(&vec);
            return
        }
    }
    println!("all tests passed!");
}