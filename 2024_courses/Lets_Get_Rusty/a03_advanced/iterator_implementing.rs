use std::collections::HashMap;

fn main() {
    let person = Person {
        first_name: "John".to_owned(),
        last_name: "Deo".to_owned(),
        occupation: "Software Engineer".to_owned(),
    };

    let mut person_iter: PersonIterator = person.into_iter();
    // let mut person_iter = person.into_iter().into_iter();

    println!("~~~ Next: {:?}", person_iter.next());

    for item in person_iter {
        println!("~~~ Item: {item}");
    }

    //
    let mut scores: HashMap<&str, i32> = HashMap::new();
    scores.insert("red team", 2);
    scores.insert("blue team", 8);
    scores.insert("green team", 6);

    let mut scores_iter = scores.iter();

    println!("~~~ Next: {:?}", scores_iter.next()); // Option<(&str, i32)>
    println!("~~~ Next: {:?}", scores_iter.next()); // Option<(&str, i32)>

    for (team, score) in &scores {
        println!("~~~ Record: {team} got {score} points");
    }
}

//
struct Person {
    first_name: String,
    last_name: String,
    occupation: String,
}

impl IntoIterator for Person {
    type Item = String;

    type IntoIter = PersonIterator;

    fn into_iter(self) -> Self::IntoIter {
        PersonIterator { values: vec![self.first_name, self.last_name, self.occupation] }
    }

    /*
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.first_name, self.last_name, self.occupation].into_iter()
    }
    */
}

//
struct PersonIterator {
    values: Vec<String>,
}

impl Iterator for PersonIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty() {
            return None;
        } else {
            Some(self.values.remove(0))
        }
    }
}
