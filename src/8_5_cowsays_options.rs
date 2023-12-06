const COWSAY: &str = r#"
\  ^__^
 \ (oo)\_______
   (__)\       )\/\
       ||----w |
       ||     ||
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


const TUX: &str = "tux";
const DRAGON: &str = "dragon";
const COW: &str = "cow";

fn prepend_spaces(s: &str, n: usize) -> String {
    let mut s = s.to_string();
    s.insert_str(0, &" ".repeat(n));
    s
}

fn print_animal(what_say: &str, message: String) {
    let indent = |s| prepend_spaces(s, message.len() + 4);
    let animal_and_say = what_say.trim()
        .split("\n")
        .map(indent)
        .collect::<Vec<String>>()
        .join("\n");
    println!("{animal_and_say}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 3 {
        // println!("two args");
        let message = args.get(2).cloned().unwrap_or_else(|| "Moo!".to_string());
        println!("< {} >", message);
        // let indent = |s| prepend_spaces(s, message.len() + 4);
        let animal = args.get(1).cloned().unwrap();

        match animal.as_str() {
            COW => print_animal(COWSAY, message),
            TUX => print_animal(TUXSAY, message),
            DRAGON => print_animal(DRAGONSAY, message),
            _ => panic!{"Unsupported animal: {}",animal}
        }

    }else if args.len() == 2 {
        let message = args.get(1).cloned().unwrap_or_else(|| "Moo!".to_string());

        println!("< {} >", message);

        print_animal(COWSAY, message);
    }else if args.len() == 1 {
        let raw_message: &str = "Moo!";
        let message = raw_message.to_string();
        println!("< {} >", message);
        print_animal(COWSAY, message);
    }

}
