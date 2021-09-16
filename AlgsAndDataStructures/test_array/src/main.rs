use arrayvec::ArrayVec;

fn main() {
    let mut big_array = ArrayVec::<_, 1000000>::new();
    for i in 0..1000001 {
        big_array.insert(i,i);
    }
}
