use std::io;
use std::io::{stdin, stdout, Write};

// fix for the future: Change from String to a Vector<&str> or something. Getting an extra sector.
// ^ bc, ends on #,--> this will get the empty  part. However it doesn't break it so :)
// ^^ On entry() --> structural fix because it requires to mess with the rebalance code and main slightly

// Gets stock entry and amt of shares
pub fn entry() ->String{
    let mut base_string = String::from("");

    // loops til 'X X' is typed and gets all the stocks
    // gets "first"  and "second" (split by whitespace)
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
    
    }

    return base_string;
}

// simple enter String contribution unit. Prints on the same line, will be converted to f32 later
pub fn enter_contribution() ->String {
    let mut amt = String::new();

    print!("$ "); 
    stdout().flush().unwrap(); // necessary to be able to type user input into the same line as "$ "

    io::stdin()
        .read_line(&mut amt)
        .expect("Failed to read \'Contribution Amount\'");
    
    return amt;
}