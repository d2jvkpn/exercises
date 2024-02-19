#![allow(dead_code)]

use chrono::{Local, SecondsFormat};
use env_logger::Builder;
use log::LevelFilter;

use std::{
    collections::HashMap,
    error, fmt,
    io::{self, BufRead, Write},
    num,
};

// RUST_LOG=error
// RUST_LOG=off
fn main() {
    // env_logger::init();

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} - {}",
                // Local::now().format("%Y-%m-%dT%H:%M:%S%:z"),
                Local::now().to_rfc3339_opts(SecondsFormat::Millis, true),
                record.level(),
                record.args(),
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    let credit_cards: HashMap<&str, &str> = HashMap::from([
        ("Amy", "1234567 04 25 123"),
        ("Tim", "1234567 06 27"),
        ("Bob", "1234567 Dec 27 123"),
    ]);

    let mut input = Default::default();
    println!("==> Enter name:"); // invalid, Tim, Bob

    io::stdin().lock().read_line(&mut input).expect("!!! failed to read line");

    match get_credit_card_info(&credit_cards, input.trim()) {
        Ok(v) => println!("\nCredit card info: {v:?}"),
        Err(e) => {
            // eprintln!("!!! error: {e:?}")
            match &e {
                CreditCardError::InvalidInput(v) => eprintln!("==> Notification: {v}"),
                CreditCardError::Other(_, _) => {
                    eprintln!("==> Notification: Something went wrong! Try again!")
                }
            }

            log::error!("\n{e:?}");
        }
    }
}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    let card = credit_cards
        .get(name)
        .ok_or(CreditCardError::InvalidInput(format!("No credit card found for {name}.")))?;

    let card = parse_card(card).map_err(|e| {
        CreditCardError::Other(Box::new(e), format!("{name}'s card could not be parsed."))
    })?;

    Ok(card)
}

fn parse_card(card: &str) -> Result<Card, PaymentError> {
    let numbers: Vec<u32> = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(PaymentError {
            source: None,
            msg: format!("Incorrect number of elements parsed. Expect {expected_len} but get {len}. Elements: {card:?}"),
        });
    }

    /*
    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();
     */

    let (number, month, year, cvv) = (numbers[0], numbers[1], numbers[2], numbers[3]);

    Ok(Card { number, exp: (year, month), cvv })
}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, PaymentError> {
    // todo!()
    let numbers = card
        .split_whitespace()
        .into_iter()
        .map(|v| {
            // num::ParseIntError
            v.parse().map_err(|_| PaymentError {
                source: None,
                msg: format!("{v:?} could not be parsed as u32"),
            })
        })
        .collect::<Result<Vec<u32>, _>>()
        .map_err(|e| PaymentError {
            source: Some(Box::new(e)),
            msg: format!("Failed to parse input as numbers. Input: {card:?}"),
        })?;

    // .collect::<Result<Vec<u32>, _>>()?;

    Ok(numbers)
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: (u32, u32),
    cvv: u32,
}

// #[derive(Debug)]
enum CreditCardError {
    InvalidInput(String),
    Other(Box<dyn error::Error>, String),
}

impl error::Error for CreditCardError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            CreditCardError::InvalidInput(_) => None,
            // convert &Box<dyn Error> to &(dyn error::Error + 'static)
            CreditCardError::Other(e, _) => Some(e.as_ref()),
        }
    }
}

impl fmt::Display for CreditCardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Credit card error: Could not retrieve credit card.")
    }
}

impl fmt::Debug for CreditCardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "{self}\nMessage: {msg}"),
            // Self::Other(e, msg) => f.debug_tuple("Other").field(msg).finish(),
            Self::Other(e, msg) => write!(f, "{self}\nMessage: {msg}\n\nCaused by:\n\t{e:?}"),
        }
    }
}

// #[derive(Debug)]
struct PaymentError {
    source: Option<Box<dyn error::Error>>,
    msg: String,
}

impl From<num::ParseIntError> for PaymentError {
    fn from(e: num::ParseIntError) -> Self {
        PaymentError { source: Some(Box::new(e)), msg: format!("failed to parse input as numbers") }
    }
}

impl error::Error for PaymentError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        // convert Option<T> or &Option<T> to Option<&T>
        self.source.as_deref()
    }
}

impl fmt::Display for PaymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Payment error: invalid payment info")
    }
}

impl fmt::Debug for PaymentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}\n\tMessage: {}", self.msg)?;

        if let Some(e) = self.source.as_ref() {
            write!(f, "\n\nCaused by:\n\t{e:?}")?;
        }

        Ok(())
    }
}

/*
pub trait Error: Debug + Display {
    // Provided methods
    fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
    fn description(&self) -> &str { ... }
    fn cause(&self) -> Option<&dyn Error> { ... }
    fn provide<'a>(&'a self, request: &mut Request<'a>) { ... }
}

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
*/
