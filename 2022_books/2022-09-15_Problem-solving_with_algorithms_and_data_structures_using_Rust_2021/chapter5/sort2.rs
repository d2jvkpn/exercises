fn main() {
    let mut data = vec![3, 4, 1, 2, 5, 8];
    println!("data = {:?}", data);

    let _ = sort2(&mut data);
    println!("data = {:?}", data);
}

fn sort2(nums: &mut [i32]) -> usize {
    let len = nums.len();
    if len < 2 {
        return 0;
    }
    let mut k = 0;

    for n in 0..len / 2 {
        let m = len - n - 1;
        let (mut k1, mut k2) = (n, m);

        for i in n..m {
            if nums[i] < nums[k1] {
                k1 = i;
            } else if nums[i] > nums[k2] {
                k2 = i;
            }
        }

        if k1 != n {
            nums.swap(n, k1);
            k += 1;
            dbg!(format!("n={}, k1={}", n, k1));
        }

        if k2 != m {
            nums.swap(m, k2);
            k += 1;
            dbg!(format!("m={}, k2={}", m, k2));
        }
    }

    k
}
