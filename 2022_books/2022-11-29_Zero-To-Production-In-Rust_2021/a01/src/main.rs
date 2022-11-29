use a01::run;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    run("0.0.0.0:8000")?.await
}
