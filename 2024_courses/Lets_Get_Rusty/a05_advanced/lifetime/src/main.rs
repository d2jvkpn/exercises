use rand::prelude::*;

fn main() {
    //
    let player1: String = String::from("player 1");
    let player2: String = String::from("player 2");

    let result: &str = first_turn(player1.as_ref(), player2.as_ref());

    println!("==> Player going first is: {}", result);

    let mut tweet = Tweet { content: "example" };
    let old_content = tweet.replace_content("replace_example");
    println!("~~~ old_content: ${old_content}");
    println!("~~~ tweet.content: {}", tweet.content);
}

fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
    let mut rng = rand::thread_rng();

    // rand::random()
    if rng.gen() {
        p1
    } else {
        p2
    }
}

struct Tweet<'a> {
    content: &'a str,
}

impl<'a> Tweet<'a> {
    fn replace_content(&mut self, content: &'a str) -> &str {
        let old_content = self.content;
        self.content = content;
        old_content
    }
}


// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifttime parameter, that lifetime is assigned to all output
//    parameters.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self,
//    the lifetime of self is assigned to all output lifetime parameters.
fn take_and_return_content<'a, 'b>(content: &'a str, content2: &'b str) -> &'a str {
	content
}
