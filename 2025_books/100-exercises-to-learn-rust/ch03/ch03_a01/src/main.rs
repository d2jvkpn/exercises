use std::mem;

fn main() {
    let ticket = Ticket {
        title: "Build a ticket system".into(),
        description: "A kanban board".into(),
        status: "Open".into(),
    };

    //let x = ticket.description;
    let is_open = ticket.is_open();

    let is_open = Ticket::is_open(&ticket);

    let config = Configuration::default();

    let ticket = Ticket::new("Ttile".into(), "Description".into(), "To-Do".into());
    let mut ticket = ticket.set_title("New title".into());

    ticket.set_description("New description".into());

    println!("ticket: {ticket:?}");

    // ch3.8
    assert_eq!(mem::size_of::<u8>(), 1);
    println!("size of Ticket: {}", std::mem::size_of::<Ticket>()); // output: 72= (3*8)*3

    // ch3.9
    // allocated in the heap
    // pointer+length+capactity in the stack
    let word = String::from("Hello");
    println!("1. {}", mem::size_of_val(&word)); // output: 24

    let s = String::new();
    println!("2. {}", mem::size_of_val(&s)); // output: 24

    let mut s = String::with_capacity(5);
    println!("3. {}", mem::size_of_val(&s)); // output: 24
    s.push_str("hello");
    println!("4. {}", mem::size_of_val(&s)); // output: 24
    println!("5. {}", mem::size_of_val(&*s)); // output: 5

    s.push_str("heyhey");
    println!("6. {}", mem::size_of_val(&s)); // output: 24
    println!("7. {}", mem::size_of_val(&*s)); // output: 11

    // ch3.11
    let y = "Hello".to_string();
    let x = "World".to_string();
    let h = "!".to_string();
    // Variables are dropped in reverse order of declaration
    drop(h);
    drop(x);
    drop(y);

	let s = "hello".to_string();
	compute(s); // ownership transfered
}

fn compute(s: String) {
	// Do something
	drop(s);
}

#[derive(Debug)]
struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Self {
        Self {
            title,
            description,
            status,
        }
    }

    fn is_open(&self) -> bool {
        self.status == "Open"
    }

    pub fn title(&self) -> &String {
        return &self.title;
    }

    pub fn description(&self) -> &String {
        return &self.description;
    }

    pub fn status(&self) -> &String {
        return &self.status;
    }

    pub fn set_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_status(&mut self, status: String) -> &mut Self {
        self.status = status;
        self
    }
}

struct Configuration {
    pub(crate) version: u32,
    active: bool,
}

impl Configuration {
    fn default() -> Configuration {
        Self {
            version: 0,
            active: false,
        }
    }
}
