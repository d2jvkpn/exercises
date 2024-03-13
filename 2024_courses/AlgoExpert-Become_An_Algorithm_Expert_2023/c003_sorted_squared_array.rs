use std::i32;

fn main() {
	let ans = &[-7, -5, -4, 3, 6, 8, 9];
	println!("==> ans: {:?}", sorted_sequared_array(ans));

	for i in 0..(ans.len() - 1) {
		assert!(ans[i] < ans[i + 1]);
	}
}

// slice is a sorted array
fn sorted_sequared_array(slice: &[i32]) -> Vec<i32> {
	if slice.len() < 2 {
		return slice.iter().map(|v| (*v).pow(2)).collect();
	}

	let mut ans = Vec::with_capacity(slice.len());
	(0..slice.len()).for_each(|_| ans.push(0));
	let (mut low, mut high) = (0, ans.len() - 1);

	for i in (0..ans.len()).rev() {
		if slice[low].abs() > slice[high].abs() {
			ans[i] = slice[low].pow(2);
			low += 1;
		} else {
			ans[i] = slice[high].pow(2);
			high -= 1;
		}
	}

	ans
}
