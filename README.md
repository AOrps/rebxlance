# rebxlance

[![Rust](https://github.com/AOrps/rebxlance/actions/workflows/rust.yml/badge.svg)](https://github.com/AOrps/rebxlance/actions/workflows/rust.yml)

Rust Project that works in Terminal to do Portfolio Rebalancing. 

## Technical Details
* Uses the Swensen Model for Asset Allocation. 
* Focused heavily on index stocks.


## To Prepare for use:
```sh
# Getting the repo
git clone https://github.com/AOrps/rebxlance.git

# Changing Directory to repo
cd rebxlance/

# Installing Any Needed 
make startup
```

##  To Run:
* For Debug Mode (Quicker Compilation)
```sh
# Cargo Method
cargo run  

# make Method (just basically calls `$ cargo run`)
make crun

```

* For Release Mode (Quicker Program (Optimized Executable))
```sh
cargo run --release
```

## To Use: 
| Start Display             | End Result
|:---------------------|:------------
|![](img/program_start.png)|![](img/program_end.png)

|Example
|:----------
|![](img/program_mid.png)

|In action:
|:---------------------
|![](img/git_init.gif)

### Disclaimer:
* I simply piped the executable through lolcat. Text will not show up in rainbow colors.  _It's called style_

## Limitations
* Does not have all the stocks, it has the base ones. Take a look into the **/src/rebalance_logic.rs** under the sector_category to see which ones the base has. Edit as you see fit. 

* Uses a basic portfolio rebalancing technique. It may not yield the top or smartest rebalancing method. **~ Greedy and Simple Approach**

* Optimized for Speed for quick results. Thus code may seem repetitive and verbose. **Better way to code this, for sure**

* Will not even begin to rebalance non-Index ETFs/Stocks. **Use Index _noobs_**


## Author 
* AOrps
