use std::io;
use std::io::{stdin, stdout, Write};

pub fn entry() ->String{
    let mut base_string = String::from("");

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut pieces = input.trim().split_whitespace();
        let first = pieces.next().unwrap();
        let second = pieces.next().unwrap();

        if first.is_empty() && second.is_empty() {
            continue;
        }

        if first == "X" && second == "X" {
            break;
        } 

        base_string.push_str(&*first);
        base_string.push('~');   
        base_string.push_str(&*second);
        base_string.push('#');
    
        println!("{} and {}", first,second);
    }
    return base_string;
}

pub fn enter_contribution() ->String {
    println!("Enter contribution amount\n$ ");
    let mut amt = String::new();

    io::stdin()
        .read_line(&mut amt)
        .expect("Failed to read \'Contribution Amount\'");
    
    return amt;
}