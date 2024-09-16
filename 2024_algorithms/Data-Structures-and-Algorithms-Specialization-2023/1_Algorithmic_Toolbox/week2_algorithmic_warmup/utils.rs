use std::{
    error,
    io::{stdin, stdout, Write},
};

pub fn read_usize(prompt: &str) -> Result<usize, Box<dyn error::Error>> {
    let mut input = String::new();

    print!("{}", prompt);
    stdout().flush()?;
    input.clear();
    stdin().read_line(&mut input)?;
    let num: usize = input.trim().parse()?;

    Ok(num)
}
