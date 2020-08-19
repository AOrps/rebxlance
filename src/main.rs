mod rebalance_logic;
mod term_ui;

/*
Uses the Swensen Model for Asset Allocation
Doesn't have stocks that don't exist and will almost certainly panic and crash.
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

    // let fast = text.clone();

    let asset = rebalance_logic::finance::core_logic(text);

    println!("Enter contribution amount");
    let contri_amt = term_ui::enter_contribution();

    asset.print_all_values();
    println!("=================================");

    let rebxlance_assets = rebalance_logic::finance::rebxlance(asset, contri_amt);
    rebxlance_assets.print_all_values();

    //rebalance_logic::finance::core_logic(text);
    // println!("Total = {}",rebalance_logic::finance::total_value(text, true));
}
