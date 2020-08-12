// This file has portfolio balancing and dividied yield
// const REST_API_KEY: &str = "408JLKWT35ALS80X";
// https://www.alphavantage.co/documentation/
// ^ doesn't use this shit


pub mod finance {
    
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

    fn parsef32name(f32string: String) ->f32 {
        let new_num: f32 = f32string.trim().parse().unwrap();
        return new_num;
    }

    fn parsef32shares(f32shares: &str) ->f32 {
        let n_num = f32shares.parse::<f32>().unwrap();
        return n_num;
    }
 



    

    pub fn total_value(base: String) ->f32{
        let mut total: f32 = 0.0;
        for chr in base.split('#') {
            // println!("{}", chr);
            let make_string_stonk = String::from(chr);

            let num = make_string_stonk.trim().split("~");

            let follow = num.clone();
            let vec: Vec<&str> = follow.collect();

            if vec.len() > 1 {
                let stonk_price = parsef32name(stonk_price_go(String::from(vec[0])));
                let share_amt: f32 = parsef32shares(vec[1]);
                let cost = stonk_price * share_amt;

                println!("{:.2} o {:.2} = {:.2}", stonk_price, share_amt, cost);

                total += cost
            }
            
        }
        println!("Total: {:.2}", total);
        return total;
        
    }


}