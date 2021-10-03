
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
    
fn swap(list: &mut Vec<i32>, i: usize, j: usize){
    let temp = list[i];
    list[i] = list[j];
    list[j] = temp;
}

fn dec_heap_size(heap: &mut Vec<i32>){
    heap[0] -= 1;
}

fn heapify(heap: &mut Vec<i32>, i: usize){
    let mut max = heap[i];
    let mut pos: usize = i;
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
    for i in (0..(heap_size(list)/2) as usize).rev(){
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
    let original_size = heap_size(list);
    for i in 1..(original_size + 1) as usize{
        let max = extract_max(list);
        let size = heap_size(list);
        list[size] = max;
        dec_heap_size(list);
    }
    list.remove(0);
}
