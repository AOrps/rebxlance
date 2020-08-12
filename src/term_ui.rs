use std::io::{stdin, stdout, Write};

pub fn entry() ->String{
    let mut total = String::from("");

    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut pieces = input.trim().split_whitespace();
        let first = pieces.next().unwrap();
        let second = pieces.next().unwrap();

        if first == "DONE" && second == "X" {
            break;
        } 

        total.push_str(&*first);
        total.push('~');   
        total.push_str(&*second);
        total.push('#');
        
        // let mdexamp = MainData(String::from("Fuck"), 345.3453);
        // println!("{} is {}", mdexamp.0, mdexamp.1);

        println!("{} and {}", first,second);



    }

    return total;
}





// pub use crate::rebalance_logic::finance;

// use piechart::{Chart, Color, Data};

// pub fn draw_pie_chart() {
//     let data = vec![
//         Data { label: "Chocolate".into(), value: 4.0, color: Some(Color::Blue), fill: '•' },
//         Data { label: "Strawberry".into(), value: 2.0, color: Some(Color::Red), fill: '▪' },
//         Data { label: "Vanilla".into(), value: 2.6, color: Some(Color::Yellow), fill: '▴' },
//     ];

//     Chart::new()
//         .radius(9)
//         .aspect_ratio(3)
//         .legend(true)
//         .draw(&data);

// }