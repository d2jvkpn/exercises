use hex::encode;
use sha1::{Digest, Sha1};

use std::io::{BufRead, BufReader};
use std::{env, error::Error, fs::File};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage:\nsha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    for line in reader.lines() {
        let line = line?;
        let password = line.trim();
        if hash_to_crack == &encode(Sha1::digest(password.as_bytes())) {
            println!("Password found: {}", &password);
            return Ok(());
        }
    }

    eprintln!("password not found in wordlist :(");
    // as almost everything is an expression, this is equivalent to return Ok(());
    Ok(())
}
