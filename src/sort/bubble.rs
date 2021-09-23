pub fn sort<T: PartialOrd>(v: &mut [T]) {
    let len = v.len();
    if len < 2 {
        return;
    }
    let mut sorted_index = len - 1;
    for _ in 0..len - 1 {
        // 切片是否有序
        let mut is_sorted = true;
        let mut last_index = 0;
        for j in 0..sorted_index {
            if v[j] > v[j + 1] {
                v.swap(j, j + 1);
                is_sorted = false;
                // 保存最后一次交换索引
                last_index = j;
            }
        }
        if is_sorted {
            break;
        }
        sorted_index = last_index;
    }
}
