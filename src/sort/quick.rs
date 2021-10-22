pub fn range_sort<T: PartialOrd + Copy>(v: &mut [T], left: usize, right: usize) {
    if left < right {
        let (mut l, mut r) = (left, right);
        let pivot = v[left];
        while l < r {
            // 从右向左找第一个小于 pivot 的值
            while l < r && v[r] >= pivot {
                r -= 1;
            }
            if l < r {
                v[l] = v[r];
                l += 1;
            }
            // 从左向右找第一个大于等于 pivot 的值
            while l < r && v[l] < pivot {
                l += 1;
            }
            if l < r {
                v[r] = v[l];
                r -= 1;
            }
        }
        v[l] = pivot;
        // 防止无符号类型值溢出
        if l > 0 {
            range_sort(v, left, l - 1);
        }
        range_sort(v, l + 1, right);
    }
}

pub fn sort<T: PartialOrd + Copy>(v: &mut [T]) {
    let len = v.len();
    if len > 1 {
        range_sort(v, 0, len - 1);
    }
}
