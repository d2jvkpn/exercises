#![allow(dead_code)]

use anyhow::Context;
use chrono::{Local, SecondsFormat};
use env_logger::Builder;
use log::LevelFilter;
use thiserror::Error;

use std::{
    collections::HashMap,
    io::{self, BufRead, Write},
};

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
                CreditCardError::Other(_) => {
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

    let card = parse_card(card)
        .with_context(|| format!("{name}'s card could not be parsed."))
        .map_err(|e| CreditCardError::Other(e))?;

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

    let (number, month, year, cvv) = (numbers[0], numbers[1], numbers[2], numbers[3]);

    Ok(Card { number, exp: (year, month), cvv })
}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, PaymentError> {
    let numbers = card
        .split_whitespace()
        .into_iter()
        .map(|v| v.parse().with_context(|| format!("{v:?} could not be parsed as u32")))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| PaymentError {
            source: Some(e),
            msg: format!("Failed to parse input as numbers. Input: {card:?}"),
        })?;

    Ok(numbers)
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: (u32, u32),
    cvv: u32,
}

#[derive(Error, Debug)]
#[error("{msg}")]
struct PaymentError {
    source: Option<anyhow::Error>,
    msg: String,
}

#[derive(Error, Debug)]
enum CreditCardError {
    #[error("{0}")]
    InvalidInput(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
