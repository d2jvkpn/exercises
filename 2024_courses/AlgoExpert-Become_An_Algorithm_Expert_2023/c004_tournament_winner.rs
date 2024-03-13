use std::collections::HashMap;

fn main() {
	let competitions = &[("HTML", "C#"), ("C#", "Python"), ("Python", "HTML")];
	let scores = &[0, 0, 1];

	let ans = tournament_winner(competitions, scores);
	dbg!(&ans);
	assert!(ans == Some("Python"));
}

fn tournament_winner<'a>(
	competitions: &'a [(&'a str, &'a str)],
	scores: &'a [usize],
) -> Option<&'a str> {
	assert!(competitions.len() == scores.len());

	let mut map = HashMap::new();

	for i in 0..competitions.len() {
		let (home_team, away_team) = competitions[i];
		let winner = if scores[i] == 0 { away_team } else { home_team };
		let value = map.entry(winner).or_insert(0);
		*value += 1;
	}

	let top = map.iter().max_by(|a, b| a.1.cmp(&b.1))?;
	Some(top.0)
}
