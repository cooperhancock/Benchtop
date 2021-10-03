//                                             //
// UTILITY FUNCTIONS FOR ALGORITHMS PLAYGROUND //
//                                             //


// returns true if comparitively sorted by the passed comparison function
pub fn is_sorted(arr: &Vec<i32>, f: fn(i32, i32) -> bool) -> bool{
    for i in 0..(arr.len()-1){
        if f(arr[i], arr[i+1]) != true{
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