use std::{fmt, fs::File, io::Write};

#[derive(Debug)]
pub enum AppError {
    ParseError(String),
    IOError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ParseError(v) => write!(f, "Parse Errror: {v:?}"),
            AppError::IOError(v) => write!(f, "IO Error: {v:?}"),
        }
    }
}

fn main() {
    let result = square("INVALID");

    match result {
        Ok(v) => println!("Result is {v:?}"),
        Err(e) => eprintln!("{e:?}"),
    }
}

fn square(val: &str) -> Result<i32, AppError> {
    let num = val
        .parse::<i32>()
        .map_err(|e| AppError::ParseError(e.to_string()))?;
    let mut f = File::open("fictional_file.txt").map_err(|e| AppError::IOError(e.to_string()))?;
    let ans = num.pow(3);

    let out = format!("Square of {num}, is {ans}");

    if let Err(e) = f.write_all(out.as_bytes()) {
        return Err(AppError::IOError(e.to_string()));
    };

    Ok(ans)
}
