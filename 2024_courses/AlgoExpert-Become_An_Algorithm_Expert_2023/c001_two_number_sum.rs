use std::collections::HashMap;

fn main() {
	let ans = two_number_sum(&[3, 5, -4, 8, 11, 1, -1, 6], 10);
	assert!(ans.is_some());

	println!("==> ans: {:?}", two_number_sum(&[3, 5, -4, 8, 11, 1, -1, 6], 10));
}

// slice contains unique elements
fn two_number_sum(slice: &[i32], sum: i32) -> Option<(i32, i32)> {
	let mut map = HashMap::with_capacity(slice.len());

	slice.iter().for_each(|v| {
		map.insert(v, true);
	});

	for v in slice {
		if *v == sum - *v {
			continue;
		}
		if map.contains_key(&(sum - *v)) {
			return Some((*v, sum - *v));
		}
	}

	None
}
