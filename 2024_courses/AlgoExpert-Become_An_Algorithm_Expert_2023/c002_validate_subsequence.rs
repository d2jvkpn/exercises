fn main() {
	assert!(valid_subsequence(&[5, 1, 22, 25, 6, -1, 8, 10, 11], &[1, 6, -1, 10]));
}

fn valid_subsequence(seq1: &[i32], seq2: &[i32]) -> bool {
	let (mut i, mut j) = (0, 0);

	while i < seq1.len() && j < seq2.len() {
		if seq1[i] == seq2[j] {
			j += 1;
		}

		i += 1;
	}

	j == seq2.len()
}
