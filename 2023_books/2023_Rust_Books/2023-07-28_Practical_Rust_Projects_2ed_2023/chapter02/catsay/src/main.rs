use clap::Parser;
use colored::Colorize;
use std::{
    error::Error,
    fs::read_to_string,
    io::{self, Read},
    path::PathBuf,
};

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    /// What does the cat say?
    message: String,

    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,

    #[clap(short = 'f', long = "file")]
    /// Load the cat picture from the specified file
    cat_file: Option<PathBuf>,

    #[clap(short = 'i', long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    // let message = std::env::args().nth(1).expect("Missing the message. Usage: catsay <message>");

    let options = Options::parse();

    let message = if options.message == "-" {
        let mut v = String::new();
        io::stdin().read_to_string(&mut v)?;
        v.trim().bright_yellow().underline().on_purple()
    } else {
        options.message.bright_yellow().underline().on_purple()
    };

    let eye = if options.dead { "x" } else { "o" };

    if message.to_lowercase() == "woof" {
        eprintln!("A cat shouldn't bark like a dog.");
    }

    match &options.cat_file {
        Some(v) => {
            let cat_template =
                read_to_string(v).map_err(|v| format!("could not read file: {:?}", v))?;
            // .expect(&format!("could not read file {:?}", v));
            let eye = format!("{}", eye.red().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}\n{}", message, &cat_picture);
        }
        None => {
            // println!("{}", message);
            println!("{}", message);
            println!(r#" \"#);
            println!(r#"  \"#);
            println!(r#"     /\_/\"#);
            // println!("    ( o o )");
            println!("    ( {eye} {eye} )", eye = eye.red().bold());
            println!("    =( I )=");
        }
    }

    Ok(())
}
