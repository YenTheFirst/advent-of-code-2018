use std::io::stdin;
fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("couldn't read input");
    let mut chars : Vec<char> = input.trim().chars().collect();

    let mut i = 0;
    while i < chars.len() {
        if i > 0 && chars[i].eq_ignore_ascii_case(&chars[i-1]) && chars[i].is_lowercase() != chars[i-1].is_lowercase() {
            chars.remove(i);
            chars.remove(i-1);
            i -= 1;
        } else {
            i += 1;
        }
    }
    println!("{}", chars.len())
}
