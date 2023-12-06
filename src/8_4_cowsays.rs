const COWSAY: &str = r#"
\  ^__^
 \ (oo)\_______
   (__)\       )\/\
       ||----w |
       ||     ||
"#;

fn prepend_spaces(s: &str, n: usize) -> String {
    let mut s = s.to_string();
    s.insert_str(0, &" ".repeat(n));
    s
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut message = if args.len() > 1 {
        args[1].to_string()
    } else {
        "Moo!".to_string()
    };

    println!("< {} >", message);
    let indent = |s| prepend_spaces(s, message.len() + 4);
    let cow = COWSAY
        .trim()
        .split("\n")
        .map(indent)
        .collect::<Vec<String>>()
        .join("\n");
    println!("{cow}");
}
