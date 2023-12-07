use application::{COWSAY, DRAGONSAY, TUXSAY};
use clap::Parser;

/// A simple cowsay clone
#[derive(Parser)]
#[clap(name = "application", about = "A simple cowsay clone")]
struct Opts {
    #[clap(short = 'a', long = "animal", default_value = "cow")]
    animal: String,

    #[clap(short = 'm', long = "message", default_value = "Moo!")]
    message: String,
}

fn prepend_spaces(s: &str, n: usize) -> String {
    let mut s = s.to_string();
    s.insert_str(0, &" ".repeat(n));
    s
}

fn main() {
    let opts = Opts::parse();
    let Opts { animal, message } = opts;

    let art = match animal.to_lowercase().as_str() {
        "dragon" => DRAGONSAY,
        "tux" => TUXSAY,
        "cow" => COWSAY,
        _ => panic!("Unknown animal: {message}"),
    };

    println!("< {} >", message);
    let indent = |s| prepend_spaces(s, message.len() + 4);
    let art = art
        .trim()
        .split('\n')
        .map(indent)
        .collect::<Vec<String>>()
        .join("\n");
    println!("{art}");
}
