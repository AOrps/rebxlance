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


pub mod finance {

    pub struct asset_alloc {
        domestic: f32,
        reits: f32,
        tips: f32,
        developed: f32,
        government: f32,
        emerging: f32,
        individual: f32,
    }

    impl asset_alloc {
        pub fn new() ->asset_alloc {
            asset_alloc {
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
            let xdomestic = String::from("domestic");
            let xreits = String::from("reit");
            let xtips = String::from("tips");
            let xdeveloped = String::from("developed");
            let xgovernment = String::from("government");
            let xemerging = String::from("emerging");
            let xindividual = String::from("individual");

            match location.to_ascii_lowercase() {
                xdomestic => self.domestic += val,
                xreits => self.reits += val,
                xtips => self.tips += val,
                xdeveloped => self.developed += val,
                xgovernment => self.government += val,
                xemerging => self.emerging += val,
                xindividual => self.individual += val,
                _ => println!("da fuck is going on"),

            }

        } 

        pub fn print_all_values(&self) {
            println!("domestic: {}", self.domestic);
            println!("reit: {}", self.reits);
            println!("tips: {}", self.tips);
            println!("developed: {}", self.developed);
            println!("government: {}", self.government);
            println!("emerging: {}", self.emerging);
        }

    }
    
    use std::collections::HashSet;

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


    

    pub fn total_value(base: String, prnt: bool) ->f32{
        let mut total: f32 = 0.0;
        for chr in base.split('#') {
            // println!("{}", chr);
            let indivi_stonk = String::from(chr);
            let total_stonk_cost = cost_per_stonk_n_share_amt(indivi_stonk);
            total += total_stonk_cost;    
        }
        if prnt {
            println!("Total: {:.2}", total);
        }
        return total;
        
    }

    fn get_stonk_name(small_base: &str) {
        let num = small_base.trim().split('~');
        let dude = num.clone(); 
    }

    pub fn core_logic(base_string: String) {
        let base = base_string.clone();
        let total = total_value(base_string, false);
        let mut ass_alloc = allocation();

        for chra in base.split('#') {
            let type_sector = sector_category(&*String::from(chra));
            let cost_sector = cost_per_stonk_n_share_amt(String::from(chra));
            if !type_sector.is_empty() {
                ass_alloc.add_to(type_sector, cost_sector);
            }
            
            ass_alloc.add_to("individual".to_owned(), cost_sector);
            

        }

        // check if stonks are in sectors and add to ass_alloc

        // ass_alloc.add_to(String::from("domestic"), 10.0);
        // ass_alloc.print_all_values();

        println!("Total = {}", total);

    }

    fn allocation() ->asset_alloc {
        let mut asset = asset_alloc::new();

        asset.add_to(String::from("domestic"), 10.0);

        return asset;
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

        if domestic_equities.contains(&stonk) {
            sector.push_str("domestic");
        }

        if emerging_markets.contains(&stonk) {
            sector.push_str("emerging");
        }

        if developed_internationl_equities.contains(&stonk) {
            sector.push_str("developed");
        }

        if government_bonds.contains(&stonk) {
            sector.push_str("government");
        }

        if tips.contains(&stonk) {
            sector.push_str("tips");
        }

        if reits.contains(&stonk) {
            sector.push_str("reit");
        }

        return sector;
        
    }

}
