use pyo3::prelude::{Python, PyModule, PyAny};
use pyo3::types::PyTuple;

/*
Uses python module named yfinance to get stock Price from stock symbol

On Rust Side uses pyo3 to use yfinance
*/

pub fn get_stonk_price(symbol: String) -> f32 {
    Python::with_gil(|py|{

        // Gets Stonk symbol
        let args = PyTuple::new(py,&[symbol]);
        
        
        // import yfinance
        let yfinance = PyModule::import(py, "yfinance").unwrap();

        // --> ticker = yfinance.Ticker(args)
        let ticker = yfinance.call1("Ticker", args).unwrap();

        // --> info = ticker.info
        let info = ticker.getattr("info").unwrap();

        // --> price = info['regularMarketPrice']
        let price : &PyAny = info.get_item("regularMarketPrice").unwrap();

        // --> Converts PyAny into Rust f32
        let price_ret : f32 = price.extract().unwrap();

       price_ret
    })
}
