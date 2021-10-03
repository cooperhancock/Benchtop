use rand::prelude::*;

pub fn quick_sort(arr: &mut Vec<i32>){
    quick_sort_(arr, 0, (arr.len()-1) as isize)
}

fn quick_sort_(arr: &mut Vec<i32>, p: isize, r: isize){
    if r<=p{
        return;
    }
    let q = partition(arr, p, r);
    quick_sort_(arr, p, q);
    quick_sort_(arr, (q+1), r);
}

fn partition(arr: &mut Vec<i32>, p: isize, r: isize) -> isize{
    let mut rng = thread_rng();
    swap(arr, rng.gen_range(p..=r) as usize, r as usize); // move pivot to end
    let pivot = arr[r as usize]; // set pivot value
    let mut i: isize = -1;
    let mut j: isize = 0;
    // from beginning of slice to pivot
    while j<r {
        if arr[j as usize] > pivot { // if bigger than pivot, increase big region
            j += 1;
        } else { // else swap with first big item
            swap(arr, (i+1) as usize, j as usize);
            i += 1; // make small region bigger to encompass new small item
            j += 1; // make big region bigger to encompass new big item at end
        }
    }
    swap(arr, r as usize, (i+1) as usize); // swap pivot with first big item to be between small and big regions
    return i // return pivot location
}

fn swap(list: &mut Vec<i32>, i: usize, j: usize){
    let temp = list[i];
    list[i] = list[j];
    list[j] = temp;
}