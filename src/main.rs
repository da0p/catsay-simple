use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Options {
    #[clap(default_value = "Meow!")]
    /// What does the cat say?
    message: String,
    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,
    #[clap(short = 'f', long = "file")]
    /// Load the cat picture fromt he specified file
    catfile: Option<std::path::PathBuf>,
    #[clap(short = 'i', long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,
}

fn main() -> Result<()> {
    let options = Options::parse();

    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
    } else {
        message = options.message;
    }
    let message = message.bright_yellow().underline().on_purple();
    let eye = if options.dead { "x" } else { "o" };
    let eye_color = format!("{}", eye.red().bold());

    match &options.catfile {
        Some(path) => print_cat_from_file(&message, &eye_color, path),
        None => print_default_cat(&message, &eye_color),
    }
}

fn print_cat_from_file(message: &str, eye_color: &str, path: &PathBuf) -> Result<()> {
    let cat_template =
        std::fs::read_to_string(path).with_context(|| format!("could not read file {:?}", path))?;
    let cat_picture = cat_template.replace("{eye}", &eye_color);
    println!("{}", message);
    println!("{}", &cat_picture);

    Ok(())
}

fn print_default_cat(message: &str, eye_color: &str) -> Result<()> {
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("    /\\_/\\");
    println!("   ( {eye_color} {eye_color} )");
    println!("   =( | )=");

    Ok(())
}