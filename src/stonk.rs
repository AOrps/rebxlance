
use pyo3::prelude::*;
use pyo3::types::PyTuple;


pub fn get_stonk_price() -> PyResult<()> {
    Python::with_gil(|py|{

        let vea = String::from("vea");
        
        let yfinance = PyModule::import(py, "yfinance")?;


        let args = PyTuple::new(py,&[vea]);

        let ticker = yfinance.call1("Ticker", args)?;

        let info = ticker.getattr("info")?;

        let price = info.get_item("regularMarketPrice")?;

        println!("{}", price);

        Ok(())
    })
}

// pub fn getStockPrice() -> PyResult<()> {
//     Python::with_gil(|py|{

//         let vea = String::from("vea");
        
//         let yfinance = PyModule::import(py, "yfinance")?;


//         let args = PyTuple::new(py,&[vea]);

//         let ticker = yfinance.call1("Ticker", args)?;

//         let info = ticker.getattr("info")?;

//         let price = info.get_item("regularMarketPrice")?;

//         println!("{}", price);

//         Ok(())
//     })
// }
