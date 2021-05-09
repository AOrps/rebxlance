mod rebalance_logic;
mod term_ui;
mod stock;
/*
Uses the Swensen Model for Asset Allocation
Doesn't have stocks that don't exist and will almost certainly panic and crash.

Note that in all reality this isn't a algorithmically complex project to improve speed.
Furthermore that it takes into consideration only index ETFs. While individuals will be calculated in total,
    it will not be part of the rebalancing. Also, in order to make it more useful one needs to all more <str> types
    into the sector_category function found in {rebalance_logic.rs}. 
    

Extra side piece, this will not do anything amazing and rebalance like a genius. Does it in a basic 
re-balancing technique. So if the contribution amount is substantially higher than the total assets. It will
eclipse and not do the "smartest rebalancing technique". 
*/

// main just delegates essentially and is a chad
fn main()  {
   
    println!("================================================================");
    println!("Type in Stonk Symbol and Amount of Shares");
    println!("\"> [STOCK_SYM] [AMT_OF_SHARES]\t\"");
    println!("================================================================");
    print!("Examples:\n");
    println!("\t \"> VTI 1.54\t\t\"");
    println!("\t \"> spyD 6 \t\t\"");
    println!("\t \"> qqq 7.50\t\t\"");
    println!("");
    println!("When done type \"X X\", currently everything else will panic :)");
    println!("================================================================");
    
    // Base String might look something like this: vea~1#vti~1#vnq~2#vwo~1#vtip~2#vgit~2#
    let text = term_ui::entry();   // Gets the base String 
    let asset = rebalance_logic::finance::core_logic(text); // gets the Asset value 

    println!(""); // Just some breathing space, everyone needs to breathe

    println!("Enter contribution amount:");
    let contri_amt = term_ui::enter_contribution(); // monthly contribution

    asset.print_all_values();  // -> prints all the asset (Current {There is no change or rebalance})    

    let rebxlance_assets = rebalance_logic::finance::rebxlance(asset, contri_amt);
    rebxlance_assets.print_all_values(); // prints the rebalance recommendation

}
