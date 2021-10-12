pub fn sort<T: PartialOrd + Copy>(v: &mut [T]) {
    for i in 1..v.len() {
        let current = v[i];
        let mut j = i;
        while j > 0 && current < v[j - 1] {
            v[j] = v[j - 1];
            j -= 1;
        }
        if j != i {
            v[j] = current;
        }
    }
}
