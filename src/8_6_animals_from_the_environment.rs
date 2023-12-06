// setx COWSAY_ANIMAL "tux"

use std::collections::HashMap;
use std::env;
use std::env::var;
use std::string::ToString;

const DRAGONSAY: &str = r#"
\
 \                       / \  //\
  \        |\___/|      /   \//  \\
   \       /0  0  \__  /    //  | \ \
    \     /     /  \/_/    //   |  \  \
          @_^_@'/   \/_   //    |   \   \
          //_^_/     \/_ //     |    \    \
       ( //) |        \///      |     \     \
     ( / /) _|_ /   )  //       |      \     _\
   ( // /) '/,_ _ _/  ( ; -.    |    _ _\.-~        .-~~~^-.
 (( / / )) ,-{        _      `-.|.-~-.           .~         `.
(( // / ))  '/\      /                 ~-. _ .-~      .-~^-.  \
(( /// ))      `.   {            }                   /      \  \
 (( / ))     .----~-.\        \-'                 .~         \  `. \^-.
            ///.----..>        \             _ -~             `.  ^-`  ^-_
              ///-._ _ _ _ _ _ _}^ - - - - ~                     ~-- ,.-~
                                                                 /.-~
"#;

const TUXSAY: &str = r#"
\   .--.
 \ |o_o |
   |:_/ |
  //   \ \
 (|     | )
/'\_   _/`\
\___)=(___/
"#;

const COWSAY: &str = r#"
\  ^__^
 \ (oo)\_______
   (__)\       )\/\
       ||----w |
       ||     ||
"#;

const DEFAULT_MESSAGE: &str = r#"Moo!"#;

fn prepend_spaces(s: &str, n: usize) -> String {
    let mut s = s.to_string();
    s.insert_str(0, &" ".repeat(n));
    s
}

fn print_cow(message: String){
    println!("< {} >", message);
    let indent = |s| prepend_spaces(s, message.len() + 4);

    let art = COWSAY
        .trim()
        .split("\n")
        .map(indent)
        .collect::<Vec<String>>()
        .join("\n");
    println!("{art}");
}

fn print_animal(animal: &String, message: String){
    println!("< {} >", message);
    let indent = |s| prepend_spaces(s, message.len() + 4);

    let art = match animal.to_lowercase().as_str() {
        "dragon" => DRAGONSAY,
        "tux" => TUXSAY,
        "cow" => COWSAY,
        _ => panic!("Unknown animal: {message}"),
    };

    let art = art
        .trim()
        .split("\n")
        .map(indent)
        .collect::<Vec<String>>()
        .join("\n");
    println!("{art}");
}

fn main() {


    // get command vars (get message)
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 {
        // get env vars
        let vars: HashMap<String, String> = env::vars().collect();

        let message = args.get(1).cloned().unwrap_or_else(|| DEFAULT_MESSAGE.to_string());
        match vars.get("COWSAY_ANIMAL") {
            Some(animal) => print_animal(animal,message),
            None => print_cow(message),
        }
    } else if args.len() == 3{
        let message = args.get(2).cloned().unwrap_or_else(|| DEFAULT_MESSAGE.to_string());
        // let indent = |s| prepend_spaces(s, message.len() + 4);
        let animal = args.get(1).cloned().unwrap().to_string();

        match animal.as_str() {
            COW => print_animal(&animal, message),
            TUX => print_animal(&animal, message),
            DRAGON => print_animal(&animal, message),
            _ => panic!{"Unsupported animal: {}",animal}
        }
    } else {
        print_cow(DEFAULT_MESSAGE.to_string());
    }
}