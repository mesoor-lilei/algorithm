fn merge<T: PartialOrd + Copy>(v: &mut [T], start: usize, middle: usize, end: usize) {
    let left_v = v[start..middle].to_vec();
    let right_v = v[middle..end].to_vec();

    let left = left_v.len();
    let right = right_v.len();

    // 合并时跟踪位置的指针
    let mut l = 0;
    let mut r = 0;
    let mut i = start;

    // 从左半边或右半边一个一个地选择较小的元素
    while l < left && r < right {
        if left_v[l] < right_v[r] {
            v[i] = left_v[l];
            l += 1;
        } else {
            v[i] = right_v[r];
            r += 1;
        }
        i += 1;
    }

    // 替换剩下的数据
    while l < left {
        v[i] = left_v[l];
        i += 1;
        l += 1;
    }
    while r < right {
        v[i] = right_v[r];
        i += 1;
        r += 1;
    }
}

fn range_sort<T: PartialOrd + Copy>(v: &mut [T], start: usize, end: usize) {
    if start < end {
        let middle = (start + end) / 2;
        range_sort(v, start, middle);
        range_sort(v, middle + 1, end);
        merge(v, start, middle + 1, end + 1);
    }
}

pub fn sort<T: PartialOrd + Copy>(v: &mut [T]) {
    let len = v.len();
    if len > 1 {
        range_sort(v, 0, len - 1);
    }
}
