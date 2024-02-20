#![allow(dead_code, unused_variables)]

fn main() {
    let students = vec!["Bogdan 3.1", "Wallace 2.3", "Lidiya 3.5", "Kyle 3.9", "Anatoliy 4.0"];

    /*
    let mut good_students: Vec<Student> = Vec::with_capacity(students.len());

    for s in students {
        let mut s = s.trim().split(' ');

        let (name, gpa) = match (s.next(), s.next()) {
            (Some(name), Some(gpa)) => (name.to_owned(), gpa.parse::<f32>()),
            _ => continue,
        };

        if let Ok(v) = gpa {
            if v >= 3.5 {
                good_students.push(Student { name, gpa: v });
            }
        }
    }
    */

    let good_students: Vec<Student> = students
        .iter()
        .map(|s| {
            let mut s = s.trim().split(' ');
            let name = s.next()?.to_owned();
            let gpa = s.next()?.parse::<f32>().ok()?;

            /*
            if gpa >= 3.5 {
                return None;
            }
            */

            Some(Student { name, gpa })
        })
        .flatten() // flattens nested structure, give up None variant
        .filter(|s| s.gpa >= 3.5)
        .collect();

    let good_students: Vec<Student> = students
        .iter()
        .filter_map(|s| {
            let mut s = s.trim().split(' ');
            let name = s.next()?.to_owned();
            let gpa = s.next()?.parse::<f32>().ok()?;

            if gpa >= 3.5 {
                return None;
            }

            Some(Student { name, gpa })
        })
        .collect();

    println!("==> good_students: {good_students:?}");
}

#[derive(Debug)]
struct Student {
    name: String,
    gpa: f32,
}
