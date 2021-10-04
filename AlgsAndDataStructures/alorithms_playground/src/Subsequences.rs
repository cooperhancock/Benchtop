use std::cmp;

// return longest increasing subsequence of vec
pub fn lis(arr: &Vec<i32>) -> usize{
    let size = arr.len();
    let mut lis_arr: Vec<i32> = Vec::with_capacity(size);
    for _i in 0..size{
        lis_arr.push(1);
    }
    for i in 1..size{
        for j in 0..i{
            // if 2 items in order and last lis < this lis + 1, inc this lis
            if arr[i] > arr[j] && lis_arr[i] < lis_arr[j] + 1 {
                lis_arr[i] = lis_arr[j] + 1;
            }
        }
    }
    let mut max = 0;
    for i in 0..size{
        max = cmp::max(max, lis_arr[i]);
    }
    return max as usize;
}

// returns min deletions to make array sorted
pub fn min_deletions(arr: &Vec<i32>) -> usize{
    return arr.len() - lis(arr);
}