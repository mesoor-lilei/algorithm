pub fn sort<T: PartialOrd + Copy>(v: &mut [T]) {
    let len = v.len();
    let mut gap = len / 2;
    while gap > 0 {
        for i in gap..len {
            let current = v[i];
            let mut j = i;
            while j >= gap && current < v[j - gap] {
                v[j] = v[j - gap];
                j -= gap;
            }
            v[j] = current;
        }
        gap /= 2;
    }
}
