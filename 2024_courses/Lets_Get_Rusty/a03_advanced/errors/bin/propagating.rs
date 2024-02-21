#![allow(dead_code)]

use std::{
    fs,
    io::{self, ErrorKind, Read},
    num,
};

fn main() -> Result<(), io::Error> {
    /*
    let content = read_file("data/not_exists.txt")?;
    println!("~~~ content: {content:?}");
     */

    match read_file("data/not_exists.txt") {
        Ok(_) => return Err(io::Error::new(ErrorKind::Other, "oh, no!")),
        Err(e) => eprintln!("~~~ expected io error: {e:?}"),
    }

    //
    let user = User { firstname: "Jane".to_owned(), lastname: "Doe".to_owned() };
    println!("~~~ get_initials: {:?}", get_initials(&user));

    //
    let result = parse_file("data/not_exists.txt");

    match result {
        Ok(v) => println!("==> got number: {v}"),
        Err(e) => {
            let err = match e {
                MyError::File(e) => e,
                MyError::NoContent => io::Error::new(ErrorKind::Other, "no content"),
                MyError::Parse(e) => io::Error::new(ErrorKind::Other, e),
            };

            return Err(err);
        }
    }

    Ok(())
}

/// read file
fn read_file(filename: &str) -> Result<String, io::Error> {
    let mut file = fs::File::open(filename)?;
    let mut content = Default::default();

    file.read_to_string(&mut content)?;

    Ok(content)
}

fn read_first_line_v1(filename: &str) -> Result<Option<String>, io::Error> {
    fs::read_to_string(filename).map(|v| v.lines().next().map(|v| v.to_owned()))
}

fn read_first_line_v2(filename: &str) -> Option<String> {
    fs::read_to_string(filename).ok().and_then(|v| v.lines().next().map(|v| v.to_owned()))
}

fn read_first_line_v3(filename: &str) -> Result<String, io::Error> {
    let err = io::Error::new(ErrorKind::Other, "no first line");

    fs::read_to_string(filename).and_then(|v| v.lines().next().map(|v| v.to_owned()).ok_or(err))
}

/// split_whitespace
#[derive(Debug)]
enum MyError {
    File(io::Error),
    Parse(num::ParseIntError),
    NoContent,
}

fn parse_file(filename: &str) -> Result<i32, MyError> {
    let content = fs::read_to_string(filename).map_err(|e| MyError::File(e))?;

    let s = content.split_whitespace().next().ok_or(MyError::NoContent)?;

    let ans = s.parse().map_err(|e| MyError::Parse(e))?;
    Ok(ans)
}

///
struct User {
    firstname: String,
    lastname: String,
}

fn get_initials(user: &User) -> Option<String> {
    let first_initials = user.firstname.chars().next()?;
    let last_initials = user.lastname.chars().next()?;

    Some(format!("{first_initials}.{last_initials}"))
}
