.SHELL = /bin/zsh

.PHONY = usage build run crun clean startup 

all = usage

usage:
	@echo "Usage:"
	@echo "make startup : install python requirements from pip"
	@echo "make build : Builds the cargo projects and runs debug executable"
	@echo "make run : Runs the built cargo project"
	@echo "make crun : Cargo run (Build and automatically run)"
	@echo "make clean : Removes an unnecessary executables / bloat"

startup:
	@echo "Installing yfinance"
	@pip install -r requirements.txt


build:
	@echo "$$ cargo build"
	@cargo build

run: build
	@echo "Running Debug Executable"
	@./target/debug/rebxlance

crun:
	@echo "$$ cargo run"
	@cargo run


clean:
ifneq (, $(wildcard ./target/.))
	@rm -rf ./target
endif