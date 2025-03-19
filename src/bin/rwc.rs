// clone of unix wc

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let target: Option<&String> = args.get(1);
    match target {
        None => {
            println!("no target supplied");
            return;
        }
        _ => (),
    }
    let contents = fs::read_to_string(target.unwrap()).expect("cannot read file");
    let mut chars = 0;
    let mut words = 0;
    let mut lines = 0;
    let mut in_word;
    for line in contents.lines() {
        chars += 1; // new line is a char
        lines += 1;
        in_word = false;
        for c in line.chars() {
            chars += 1;
            if c != ' ' && in_word == false {
                words += 1;
                in_word = true;
            } else if c == ' ' && in_word == true {
                in_word = false
            }
        }
    }
    println!("{} {} {} {}", lines, words, chars, target.unwrap());
}
