#!/usr/bin/env python3
import sys
import json
import yfinance as yf

stonk = lambda symbol: yf.Ticker(symbol).info 
stonkPrice = lambda name: print(stonk(name)['regularMarketPrice'])

if __name__ == "__main__":
    try:
        tic = sys.argv[1].upper()
        print(tic)
        stonkPrice(tic)
    except:
        print(-1)  # Stock Prices can't be negative