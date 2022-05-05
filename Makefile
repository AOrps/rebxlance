OUTPUT=main.o

all: compile

compile: clean
	@go build -o $(OUTPUT) main.go

run: compile
	@./$(OUTPUT)

clean:
ifneq (,$(wilcard ./$(OUTPUT)))
	@rm ./$(OUTPUT)
endif