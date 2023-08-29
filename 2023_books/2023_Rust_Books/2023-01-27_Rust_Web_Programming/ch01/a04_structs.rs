#![allow(dead_code)]

#[derive(Debug, Default)]
struct Human {
    name: String,
    age: i8,
    current_thought: Option<String>,
    friend: Friend,
}

impl Human {
    fn new(name: &str, age: i8) -> Human {
        return Human {
            name: name.to_string(),
            age: age,
            current_thought: None,
            friend: Friend::NIL,
        };
    }

    fn into_friend(self) -> Friend {
        Friend::HUMAN(Box::new(self))
    }

    fn with_thought(mut self, thought: &str) -> Human {
        self.current_thought = Some(thought.to_string());
        return self;
    }
    fn with_friend(&mut self, friend: Friend) -> &mut Human {
        self.friend = friend;
        return self;
    }
}

#[derive(Debug)]
enum Friend {
    HUMAN(Box<Human>),
    NIL,
}

impl Default for Friend {
    fn default() -> Self {
        Self::NIL
    }
}

fn main() {
    let another_developer = Human {
        name: "Caroline Morton".to_string(),
        age: 30,
        current_thought: Some("I need to code!!".to_string()),
        // friend: Friend::NIL,
        ..Default::default()
    };

    //    let developer = Human {
    //        name: "Maxwell Flitton".into(),
    //        age: 32,
    //        current_thought: None,
    //        // friend: Friend::HUMAN(Box::new(another_developer)),
    //        friend: another_developer.into_friend(),
    //    };
    // another_developer was moved

    let mut developer = Human::new("Maxwell Flitton", 32).with_thought("I love rust");
    developer.with_friend(another_developer.into_friend());

    match &developer.friend {
        Friend::HUMAN(data) => println!("{}", data.name),
        Friend::NIL => {}
    }
}
