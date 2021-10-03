

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