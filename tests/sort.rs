use std::fmt::Debug;
use std::time::Instant;

use rand::seq::SliceRandom;

fn repeat_data(len: usize, n: usize) -> Vec<usize> {
    (0..len).collect::<Vec<_>>().repeat(n)
}

fn shuffle<T>(data: &mut [T]) {
    data.shuffle(&mut rand::thread_rng());
}

fn data_repeat(len: usize, n: usize) -> Vec<usize> {
    let mut v = Vec::with_capacity(len * n);
    for i in 0..len {
        for _ in 0..n {
            v.push(i);
        }
    }
    v
}

macro_rules! sort {
    ($param:expr; $( $name:ident ),+) => {{
        $(
            // 打乱数组，防止使用上次已排好的数组
            shuffle($param.0);
            let start = Instant::now();
            algorithm::sort::$name::sort($param.0);
            println!("{:9} 耗时 {:?}", stringify!($name), start.elapsed());
            assert_eq!($param.0, $param.1);
        )+
    }}
}

#[test]
fn test() {
    fn sort<T: Copy + Debug + PartialOrd>(param: (&mut [T], &[T])) {
        sort!(param; bubble, selection, insertion, shell, quick, merge);
    }

    // 0 ~ 10 数组重复 3 次
    sort((&mut repeat_data(10, 3), &data_repeat(10, 3)));
    // 单个元素数组
    sort::<u8>((&mut [0], &[0]));
    // 空数组
    sort::<()>((&mut [], &[]));
}
