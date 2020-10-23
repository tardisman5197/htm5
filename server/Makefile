.PHONY = all build install run start stop
.DEFAULT_GOAL := all

all: install build

install:
	@echo "Installing the server"

build:
	@echo "Building the server"
	sudo docker build -t server .
	@echo "Built Server"

run:
	@echo "Running the server"
	sudo docker run -t server

start:
	@echo "Starting the server"
	sudo docker run -d -t server

stop:
	@echo "Stopping the server"
	sudo docker stop server