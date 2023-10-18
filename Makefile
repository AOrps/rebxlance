SHELL = /bin/sh
OUTPUT = rebxlance

all: compile

compile:
	@echo "=========== BUILD ==========="
	go build

legacy-compile: clean
	@go build -o $(OUTPUT) main.go

run: compile
	@echo "============ RUN ============"
	echo -n "Running... "
	./$(OUTPUT)

clean:
	@echo "=========== CLEAN ==========="
ifneq (,$(wildcard ./$(OUTPUT)))
	rm ./$(OUTPUT)
endif
ifneq (,$(wildcard ./*.o))
	rm ./*.o
endif
