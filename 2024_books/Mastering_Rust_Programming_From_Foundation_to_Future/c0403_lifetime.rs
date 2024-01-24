#![allow(dead_code)]

fn main() {
    // println!("Hello, wrold!");

    let text = String::from("Hello Evol");
    let holder;

    {
        holder = Holder { data: &text };
    }

    println!("~~~ data in holder: {}", holder.data);
    println!("~~~ first word: {:?}", first_word(&text));
}

struct Holder<'a> {
    data: &'a str,
}

impl<'a> Holder<'a> {
    pub fn new(data: &'a str) -> Self {
        Self { data }
    }
}

// the lifetime annotation 'a is used to specify that the process method takes an input reference
// with the same lifetime as the trait itself.
trait StringProcessor<'a> {
    fn process(&self, input: &'a str) -> &'a str;
}

fn first_word(s: &str) -> Option<&str> {
    s.split_whitespace().next()
}

struct RefHolder<'a, T> {
    data: &'a T,
}

enum RefOrValue<'a, T> {
    Ref(&'a T),
    Value(T),
}
