
// finds v in arr sorted in monotonically increasing order
// returns index or -1 if not in array
fn binary_search_(arr: &Vec<i32>, p: usize, q: usize, v: i32) -> isize{
    let m = (p+q)/2;
    //println!("p,q,v,m = {},{},{},{}", p, q, v, m);
    if arr[m] == v {
        return m as isize;
    }
    else if q-p == 0 {
        return -1;
    }
    else if v < arr[m] {
        return binary_search_(arr, p, m, v);
    }
    else {
        return binary_search_(arr, m+1, q, v);
    }
}

pub fn binary_search(arr: &Vec<i32>, v: i32) -> isize{
    return binary_search_(arr, 0, arr.len()-1, v);
}