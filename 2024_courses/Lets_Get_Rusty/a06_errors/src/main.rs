#![allow(dead_code)]

use std::{collections::HashMap, error, fmt, io, num};

// RUST_LOG=error
// RUST_LOG=off
fn main() {
    env_logger::init();

    let credit_cards: HashMap<&str, &str> = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27 123"),
        ("bob", "1234567 12 27 123"),
    ]);

    let mut input = Default::default();
    println!("==> Enter name:");

    io::stdin().read_line(&mut input).expect("!!! failed to read line");

    match get_credit_card_info(&credit_cards, input.trim()) {
        Ok(v) => println!("==> Credit card info: {v:?}"),
        Err(e) => {
            // eprintln!("!!! error: {e:?}")
            match e {
                CreditCarError::InvlaidInput(ref v) => eprintln!("!!! invaid input: {v}"),
                CreditCarError::Other(_, _) => eprintln!("!!! something went wrong!"),
            }

            log::error!("{e:?}");
        }
    }
}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCarError> {
    let card =
        credit_cards.get(name).ok_or(CreditCarError::InvlaidInput("No credit card".into()))?;

    let card =
        parse_card(card).map_err(|e| CreditCarError::Other(Box::new(e), format!("{}", name)))?;

    Ok(card)
}

fn parse_card(card: &str) -> Result<Card, ParsePaymentError> {
    let numbers: Vec<u32> = parse_card_numbers(card)?;

    if numbers.len() != 4 {
        return Err(ParsePaymentError {
            source: None,
            msg: Some(format!("incorrect number of elements parsed. expect 4 but got {:?}", card)),
        });
    }

    /*
    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();
     */

    let (cvv, year, month, number) = (numbers[0], numbers[1], numbers[2], numbers[3]);

    Ok(Card { number, exp: (year, month), cvv })
}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentError> {
    // todo!()
    let numbers = card
        .split_whitespace()
        .into_iter()
        .map(|v| {
            v.parse().map_err(|_| ParsePaymentError { source: None, msg: Some(format!("{v}")) })
        })
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e| ParsePaymentError {
            source: Some(Box::new(e)),
            msg: Some(format!("failed to parse input as numbers")),
        })?;

    Ok(numbers)
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: (u32, u32),
    cvv: u32,
}

#[derive(Debug)]
enum CreditCarError {
    InvlaidInput(String),
    Other(Box<dyn error::Error>, String),
}

#[derive(Debug)]
struct ParsePaymentError {
    source: Option<Box<dyn error::Error>>,
    msg: Option<String>,
}

impl From<num::ParseIntError> for ParsePaymentError {
    fn from(e: num::ParseIntError) -> Self {
        ParsePaymentError { source: Some(Box::new(e)), msg: None }
    }
}

impl error::Error for ParsePaymentError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_deref()
    }
}

impl fmt::Display for ParsePaymentError {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(w, "parsing payment error: invalid payment info")
        // w.write_str("parsing payment error: invalid payment info")
    }
}

///
/*
pub trait Error: Debug + Display {
    // Provided methods
    fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
    fn description(&self) -> &str { ... }
    fn cause(&self) -> Option<&dyn Error> { ... }
    fn provide<'a>(&'a self, request: &mut Request<'a>) { ... }
}
*/

enum APIError {
    UserInput(String),
    Internal(Box<dyn error::Error>),
}

struct ServerError {
    status_code: u8,
    msg: String,
    source: Box<dyn error::Error>,
    err_type: APIError,
}
