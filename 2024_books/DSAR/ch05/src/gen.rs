pub fn is_sorted<T: PartialOrd>(vec: &[T]) -> bool {
    vec.windows(2).all(|w| w[0] <= w[1])
}

// 生成随机数据集
pub fn generate_random_data(size: usize) -> Vec<i32> {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..1024)).collect()
}

// 生成几乎有序的数据集
pub fn generate_nearly_sorted_data(size: usize) -> Vec<i32> {
    let mut data: Vec<i32> = (0..size as i32).collect();
    if size > 1 {
        data[size / 2] = data[size / 2] + 1; // 打乱一个元素
    }
    data
}

// 生成反向排序的数据集
pub fn generate_reverse_sorted_data(size: usize) -> Vec<i32> {
    (0..size as i32).rev().collect()
}

// 生成包含重复元素的数据集
pub fn generate_with_duplicates_data(size: usize) -> Vec<i32> {
    let mut data = Vec::with_capacity(size);
    for i in 0..size {
        data.push(i as i32 % 3); // 生成重复的元素
    }
    data
}
