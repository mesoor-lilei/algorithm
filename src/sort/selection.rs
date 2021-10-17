pub fn sort<T: PartialOrd>(v: &mut [T]) {
    let len = v.len();
    if len < 2 {
        return;
    }
    for i in 0..len - 1 {
        let mut min_index = i;
        for j in (i + 1)..len {
            if v[j] < v[min_index] {
                min_index = j;
            }
        }
        v.swap(i, min_index);
    }
}
