mod rebalance_logic;
mod term_ui;

/*
Uses the Swensen Model for Asset Allocation
*/

fn main() {
    println!("================================================================");
    println!("Type in Stonk Symbol and Amount of Shares");
    println!("\"> [STOCK_SYM] [AMT_OF_SHARES]\t\"");
    println!("================================================================");
    print!("Examples:\n");
    println!("\t \"> PFE 1.54\t\t\"");
    println!("\t \"> ibM 6 \t\t\"");
    println!("\t \"> qqq 7.50\t\t\"");
    println!("");
    println!("When done type \"X X\", currently everything else will panic :)");
    println!("================================================================");
    
    
    let text = term_ui::entry();
    println!("{}", text);

    //rebalance_logic::finance::core_logic(text);
    // println!("Total = {}",rebalance_logic::finance::total_value(text, true));
}
