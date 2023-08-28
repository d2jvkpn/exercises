mod m7_async;

const OUR_COURSE: &str = "Rust with AutoGPT";

#[tokio::main]
async fn main() {
    println!("Welcome to this course on {}!", OUR_COURSE);
}
