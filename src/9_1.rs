const INDENT_SIZE: usize = 4;
fn print_indented(string_slice: &str, indentation_depth: usize) {
    // let target_string : () = string_slice.to_string().insert_str(0, &"1".repeat(indentation_depth as usize+20)).clone();
    let mut target_string:String = String::from(string_slice);
    target_string.insert_str(0,&" ".repeat(indentation_depth*INDENT_SIZE));

    println!("{}", target_string)
}

fn eprint_indented(string_slice: &str, indentation_depth: usize) {
    // let target_string : () = string_slice.to_string().insert_str(0, &"1".repeat(indentation_depth as usize+20)).clone();
    let mut target_string:String = String::from(string_slice);
    target_string.insert_str(0,"Error: ");
    target_string.insert_str(0,&" ".repeat(indentation_depth*INDENT_SIZE));

    eprintln!("{}", target_string)
}

fn main() {
    // print_indented("Hello!", 0);
    print_indented("Hello indent!", 1);
    eprint_indented("Hello error!", 2);
}
