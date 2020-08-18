// This file has portfolio balancing and dividied yield
// const REST_API_KEY: &str = "408JLKWT35ALS80X";
// https://www.alphavantage.co/documentation/
// ^ doesn't use this shit

/*
Domestic - 30%
REITs - 20%
TIPS - 15%
Developed_International_Equities - 15%
Government Bonds - 15%
Emerging - 5%
*/

#[allow(dead_code)]
#[allow(unreachable_patterns)]
#[allow(unused_variables)]

pub mod finance {

    const DOMESTIC_SWENSEN: f32 = 30.0;
    const DEVELOPED_SWENSEN: f32 = 15.0;
    const TIPS_SWENSEN: f32 = 15.0;
    const GOVERNMENT_SWENSEN: f32 = 15.0;
    const REIT_SWENSEN: f32 = 20.0;
    const EMERGING_SWENSEN: f32 = 5.0;
    const SECTOR: f32 = 6.0;

    use std::collections::HashSet;

    pub struct RebalanceBoard {
        contribution_amt: f32,
        amt_domestic: f32,
        amt_reits: f32,
        amt_tips: f32,
        amt_developed: f32,
        amt_government: f32,
        amt_emerging: f32,
    }
    impl RebalanceBoard {
        pub fn new() -> RebalanceBoard {
            RebalanceBoard {
                contribution_amt: 0.0,
                amt_domestic: 0.0,
                amt_reits: 0.0,
                amt_tips: 0.0,
                amt_developed: 0.0,
                amt_government: 0.0,
                amt_emerging: 0.0,
            }
        }

        pub fn custom_new(contribution_amt: f32, amt_domestic: f32, amt_reits: f32, amt_tips: f32, amt_developed: f32, amt_government: f32, amt_emerging: f32) ->RebalanceBoard {
            RebalanceBoard {
                contribution_amt,
                amt_domestic,
                amt_reits,
                amt_tips, 
                amt_developed, 
                amt_government,
                amt_emerging,
            }
        }

        pub fn change_to(&mut self, location: String, val: f32) {
            if location == String::from("contribution") {
                self.contribution_amt = val;
            } else if location == String::from("domestic") {
                self.amt_domestic = val;
            } else if location == String::from("reit") {
                self.amt_reits = val;
            } else if location == String::from("tips") {
                self.amt_tips = val;
            } else if location == String::from("developed") {
                self.amt_developed = val;
            } else if location == String::from("government") {
                self.amt_government = val;
            } else if location == String::from("emerging") {
                self.amt_emerging = val;
            } else {
                println!("da fuq is going on");
            }
        }
    }

    pub struct AssetAlloc {
        domestic: f32,
        reits: f32,
        tips: f32,
        developed: f32,
        government: f32,
        emerging: f32,
        individual: f32,
    }

    impl AssetAlloc {
        pub fn new() ->AssetAlloc {
            AssetAlloc {
                domestic: 0.0,
                reits: 0.0,
                tips: 0.0,
                developed: 0.0,
                government: 0.0,
                emerging: 0.0,
                individual: 0.0,
            }
        }

        pub fn add_to(&mut self, location: String, val: f32) {

            // println!("loc: {}", location);

            if location == String::from("domestic") {
                self.domestic += val;
            } else if location == String::from("reit") {
                self.reits += val;
            } else if location == String::from("tips") {
                self.tips += val;
            } else if location == String::from("developed") {
                self.developed += val;
            } else if location == String::from("government") {
                self.government += val;
            } else if location == String::from("emerging") {
                self.emerging += val;
            } else if location == String::from("individual") {
                self.individual += val;         
            } else {
                println!("Da fuck is gonna happen");
            }

        } 

        pub fn print_all_values(&self) {
            println!("\tdomestic: {}", self.domestic);
            println!("\treit: {}", self.reits);
            println!("\ttips: {}", self.tips);
            println!("\tdeveloped: {}", self.developed);
            println!("\tgovernment: {}", self.government);
            println!("\temerging: {}", self.emerging);
            println!("\tindividual: {}", self.individual);
        }

        pub fn total(&self) ->f32 {
            let total: f32 = self.domestic + self.reits + self.tips + self.developed + self.government + self.emerging;
            return total;
        }

    }

    
    pub fn stonk_price_go(stonk: String) ->String{
        // will return the Current Price of the Stock from a Go Script
        let go_script = format!("./stonk {}", stonk);
        
        let mut cmd_run = std::process::Command::new("bash");
        cmd_run.arg("-c")
                .arg(go_script)
                .output()
                .expect("error with go_script within Rust");


        match cmd_run.output() {
            Ok(o) => {
                unsafe {
                    return format!("{}", String::from_utf8_unchecked(o.stdout));
                }
            }
            Err(e) => {
                return format!("NO. There was an error, FuckNut: {}", e);
            }
        }

    }
    
    fn cost_per_stonk_n_share_amt(indivi: String) ->f32 {
        let mut cost: f32 = 0.0;

        let num = indivi.trim().split("~");

        let follow = num.clone();
        let vec: Vec<&str> = follow.collect();

        if vec.len() > 1 {
            let stonk_price = parsef32name(stonk_price_go(String::from(vec[0])));
            let share_amt: f32 = parsef32shares(vec[1]);
            cost += stonk_price * share_amt;
        }
 
        return cost;
    }

    fn get_stonk_name(small_base: &str) ->String {
        let num = small_base.trim().split('~');
        let dude = num.clone(); 
        let vec: Vec<&str> = dude.collect();

        let stonk_name = String::from(vec[0]);

        return stonk_name;

    }

    fn is_over(percent: f32, swensen_const: f32) -> bool {
        if percent > swensen_const {
            return false;
        } else {
            return true;
        }
    }

    pub fn core_logic(base_string: String) ->AssetAlloc {
        let base = base_string.clone();
        let mut asset_alloc = AssetAlloc::new();

        for chra in base.split('#') {
            let type_sector = sector_category(&*get_stonk_name(chra));
            println!("Type sector : {}", type_sector);
            let cost_sector = cost_per_stonk_n_share_amt(String::from(chra));

            if !type_sector.is_empty() {

                asset_alloc.add_to(type_sector, cost_sector);
            } else {
                asset_alloc.add_to("individual".to_owned(), cost_sector);
            }

        }

        return asset_alloc;
    }

    pub fn rebxlance(curr_assets: AssetAlloc, contri_amt: String) ->RebalanceBoard {

        let mut rebxlance_board = RebalanceBoard::new();

        let total = curr_assets.total();
        let month_money = parsef32name(contri_amt);

        let mut percentage_spread: f32 = 0.0;
        let mut percentage_for_each: f32 = 0.0;
        let mut sectors_under = HashSet::new();
        let mut divvy: f32 = 0.0; // number must start at six because that is the number of sectors

        let domestic_percentage = percentage(curr_assets.domestic, total);
        let reits_percentage = percentage(curr_assets.reits, total);
        let tips_percentage = percentage(curr_assets.tips, total);
        let developed_percentage = percentage(curr_assets.developed, total); 
        let government_percentage = percentage(curr_assets.government, total);
        let emerging_percentage = percentage(curr_assets.emerging, total);

        if is_over(domestic_percentage, DOMESTIC_SWENSEN) {
            percentage_spread += DOMESTIC_SWENSEN;
            divvy += 1.0;
        } else {
            sectors_under.insert("domestic".to_ascii_uppercase());
        }

        if is_over(reits_percentage, REIT_SWENSEN) {
            percentage_spread += REIT_SWENSEN;
            divvy += 1.0;
        }

        if is_over(tips_percentage, TIPS_SWENSEN) {
            percentage_spread += TIPS_SWENSEN;
            divvy += 1.0;
        }

        if is_over(developed_percentage, DEVELOPED_SWENSEN) {
            percentage_spread += DEVELOPED_SWENSEN;
            divvy += 1.0;
        }

        if is_over(government_percentage, GOVERNMENT_SWENSEN) {
            percentage_spread += GOVERNMENT_SWENSEN;
            divvy += 1.0;
        }

        if percentage_spread != 0.0 {
            let to_val = SECTOR - divvy;
            percentage_for_each += percentage_spread/to_val;
        }



        return rebxlance_board;
    }


    fn percentage(val: f32, total: f32) ->f32 {
        // gets percentage to and sets precision to 2 decimal places
        let percent = (val/total) * 100.0; 
        let percent_string = format!("{:.2}", percent); 
        return parsef32name(percent_string);
    }

    fn parsef32name(f32string: String) ->f32 {
        let new_num: f32 = f32string.trim().parse().unwrap();
        return new_num;
    }

    fn parsef32shares(f32shares: &str) ->f32 {
        let n_num = f32shares.parse::<f32>().unwrap();
        return n_num;
    }

    
    fn sector_category(stonk: &str) ->String {
        // sector_catergory - takes into consideration index funds (not Individual)
        // will be an empty string if so
        let stonk = String::from(stonk).to_ascii_uppercase();
        let mut sector =  String::new();

        /*
        Domestic - 30%
        REITs - 20%
        TIPS - 15%
        Developed_International_Equities - 15%
        Government Bonds - 15%
        Emerging - 5%
        */

        let domestic_equities: HashSet<&'static str> = 
        ["SPY", "ITOT", "VTI", "VTSAX", "VOO", "VII", "QQQ", "IWF"]
        .iter().cloned().collect();

        let emerging_markets: HashSet<&'static str> = 
        ["SCHE","ERUS","RSX","RSXJ"]
        .iter().cloned().collect();


        let developed_internationl_equities: HashSet<&'static str> = 
        ["VEA","SCHP","SPDW","GOEA"]
        .iter().cloned().collect();


        let government_bonds: HashSet<&'static str> = 
        ["EDV","TLT","ZROZ"]
        .iter().cloned().collect();


        let tips: HashSet<&'static str> =
        ["TIPX", "SCHP", "SPIP"]
        .iter().cloned().collect();

        let reits: HashSet<&'static str> = 
        ["SAFE", "EPRT", "GLPI", "DLR"]
        .iter().cloned().collect();

        if domestic_equities.contains(&*stonk) {
            sector.push_str("domestic");
        }

        if emerging_markets.contains(&*stonk) {
            sector.push_str("emerging");
        }

        if developed_internationl_equities.contains(&*stonk) {
            sector.push_str("developed");
        }

        if government_bonds.contains(&*stonk) {
            sector.push_str("government");
        }

        if tips.contains(&*stonk) {
            sector.push_str("tips");
        }

        if reits.contains(&*stonk) {
            sector.push_str("reit");
        }

        return sector;
    }

}
