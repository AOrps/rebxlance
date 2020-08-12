mod rebalance_logic;
mod term_ui;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct stonk {
    name: String,
    price: f32,
    shares: f32,
    type_category: String,
}

fn main() {
    println!("Type in Stonk Symbol and Amount of Shares");
    println!("=========================================");
    print!("Example: \t");
    println!("\"> PFE 1.54\"");
    println!("=========================================");
    
    let text = term_ui::entry();
    println!("{}", text);
    println!("{}",rebalance_logic::finance::total_value(text));
}
