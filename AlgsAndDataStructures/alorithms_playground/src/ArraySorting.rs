use rand::prelude::*;

use crate::Utils::vec_print;

// swaps 2 elements in vec
fn swap(list: &mut Vec<i32>, i: usize, j: usize){
    let temp = list[i];
    list[i] = list[j];
    list[j] = temp;
}

// QUICK SORT //

pub fn quick_sort(arr: &mut Vec<i32>){
    quick_sort_(arr, 0, (arr.len()-1) as isize)
}

fn quick_sort_(arr: &mut Vec<i32>, p: isize, r: isize){
    if r<p{
        return;
    }
    let q = partition(arr, p, r);
    quick_sort_(arr, p, q-1);
    quick_sort_(arr, q+1, r);
}

// fn partition(arr: &mut Vec<i32>, p: isize, r: isize) -> isize{
//     let mut rng = thread_rng();
//     swap(arr, rng.gen_range(p..=r) as usize, r as usize); // move pivot to end
//     let pivot = arr[r as usize]; // set pivot value
//     let mut i: isize = p-1;
//     let mut j: isize = p;
//     // from beginning of slice to pivot
//     while j<r {
//         if arr[j as usize] > pivot { // if bigger than pivot, increase big region
//             j += 1;
//         } else { // else swap with first big item
//             swap(arr, (i+1) as usize, j as usize);
//             i += 1; // make small region bigger to encompass new small item
//             j += 1; // make big region bigger to encompass new big item at end
//         }
//     }
//     swap(arr, r as usize, (i+1) as usize); // swap pivot with first big item to be between small and big regions
//     return i // return pivot location
// }

fn partition(arr: &mut Vec<i32>, p: isize, r: isize) -> isize{
    let mut rng = thread_rng();
    swap(arr, rng.gen_range(p..=r) as usize, r as usize); // move pivot to end
    let x = arr[r as usize];
    let mut i = p-1;
    for j in p..r{
        if arr[j as usize] <= x{
            i = i+1;
            swap(arr, i as usize, j as usize);
        }
    }
    swap(arr, (i+1) as usize, r as usize);
    return i+1;
}

// not sorting but actually selection //

// finds kth element in vec (purposely designed iteratively for a class :/ )
pub fn randomized_select(arr: &mut Vec<i32>, p: isize, r: isize, k: isize) -> i32{
    let mut i = p;
    let mut j = r;
    let mut k_ = k;
    while j > i {
        let q = partition(arr, i, j);
        vec_print(arr);
        println!("q {}", q);
        if q == k_ {
            return arr[q as usize];
        }
        else if (q-i+1) >= k_ {
            j = q;
        }
        else{
            i = q+1;
            k_ = k_ - (q-i+1);
        }
        println!("i, j {}, {}", i, j);
    }
    return arr[i as usize];
}

// MERGE SORT //



// merges 2 sorted adjacent slices together
// [p....q|q+1....r]
fn merge(arr: &mut Vec<i32>, p: usize, q: usize, r: usize){
    let len1 = q - p + 1;
    let len2 = r - q;
    let mut vecL: Vec<i32> = Vec::with_capacity(len1+1);
    let mut vecR: Vec<i32> = Vec::with_capacity(len2+1);
    for i in 0..(len1) {
        vecL.push(arr[p+i]);
    }
    for j in 0..(len2) {
        vecR.push(arr[q+1+j]);
    }
    vecL.push(i32::MAX);
    vecR.push(i32::MAX);
    let mut i = 0;
    let mut j = 0;
    for k in p..(r+1){
        if vecL[i] <= vecR[j]{
            arr[k] = vecL[i];
            i += 1;
        }else{
            arr[k] = vecR[j];
            j += 1;
        }
    }
}

// sorts list into monotonically increasing order
fn merge_sort_(arr: &mut Vec<i32>, p: usize, r: usize){
    if r<=p {
        return;
    }
    let q = (p+r)/2;
    merge_sort_(arr, p, q);
    merge_sort_(arr, q+1, r);
    merge(arr, p, q, r);
}

pub fn merge_sort(arr: &mut Vec<i32>){
    merge_sort_(arr, 0, arr.len()-1)
}


// HEAP SORT //


fn print_heap(heap: Vec<i32>){
    let mut level = 1;
    for i in 1..(heap_size(&heap)+1){
        print!("{} ", heap[i as usize]);
        if i == ((2_usize.pow(level))-1){
            println!("");
            level += 1;
        }
    }
}

fn heap_size(heap: &Vec<i32>) -> usize{
    return heap[0] as usize;
}

fn left(i: usize) -> usize{
    return 2*i;
}
    
fn right(i: usize) -> usize{
    return 2*i + 1;
}
    
fn parent(i: usize) -> usize{
    return i/2;
}

fn dec_heap_size(heap: &mut Vec<i32>){
    heap[0] -= 1;
}

fn heapify(heap: &mut Vec<i32>, i: usize){
    let mut max = heap[i];
    let mut pos: usize = i;
    println!("size should be {}, is actually {}", heap_size(heap), heap.len());
    if left(i) <= heap_size(heap) as usize && heap[left(i)] > max{
        max = heap[left(i)];
        pos = left(i);
    }
    if right(i) <= heap_size(heap) as usize && heap[right(i)] > max{
        max = heap[right(i)];
        pos = right(i);
    }
    if pos != i{
        swap(heap, i, pos);
        heapify(heap, pos);
    }
}

fn make_heap(list: &mut Vec<i32>){
    list.insert(0, list.len() as i32);
    for i in (1..(heap_size(list)/2) as usize).rev(){
        heapify(list, i);
    }
}

fn extract_max(heap: &mut Vec<i32>) -> i32{
    if heap_size(heap) == 1{
        return heap[1];
    }
    let max = heap[1];
    heap[1] = heap[heap_size(heap)];
    heapify(heap, 1);
    return max;
}

pub fn heap_sort(list: &mut Vec<i32>){
    make_heap(list);
    println!("heap made");
    let original_size = heap_size(list);
    for i in 1..(original_size + 1) as usize{
        let max = extract_max(list);
        let size = heap_size(list);
        list[size] = max;
        dec_heap_size(list);
    }
    list.remove(0);
    println!("heap sorted");
}
