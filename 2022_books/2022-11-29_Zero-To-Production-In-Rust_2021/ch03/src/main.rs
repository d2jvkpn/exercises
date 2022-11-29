use ch03::run;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let addr = "0.0.0.0:8000";
    println!(">>> HTTP listening on {}", addr);
    run(addr)?.await
}

#[cfg(test)]
mod tests {
    #[test]
    fn t_plus() {
        let ans = 2 + 2;
        println!("ans = {}", ans);
        assert_eq!(ans, 4);
    }

    #[test]
    fn t_minus() {
        let ans = 4 - 2;
        println!("ans = {}", ans);
        assert_eq!(ans, 2);
    }
}
