#![allow(dead_code)]

#[derive(Debug, Default, Clone)]
struct Person {
    pub name: String,
    pub age: u16,
}

fn main() {
    let person = Person { name: "Rover".into(), age: 24 };
    println!(">>> {:?}, {:?}", person, type_name_of(&person));

    let opt = Some(Box::new(person)); // Option<Box<Person>>
    call_person(opt);
    call_person(None);

    let person = Person { name: "Rover".into(), age: 24 };
    let mut opt = Some(Box::new(person));
    let p1 = opt.take();
    println!(">>> {:?}, {:?}", opt, p1); // opt.is_none(), p1.is_some()
}

fn call_person(mut item: Option<Box<Person>>) {
    //    let mut person = item.as_mut().unwrap();
    let mut person = if let Some(v) = item.as_mut() {
        v
    } else {
        return;
    }; // mut Box<Person>

    person.age = 42;
    println!("    {:?}, {:?}", person, type_name_of(&person));

    let person = &*person; // Box<Person>
    println!("    {:?}, {:?}", person, type_name_of(&person));
}

fn type_name_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}
