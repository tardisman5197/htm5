.PHONY = all build install run start stop
.DEFAULT_GOAL := all

all: install build

install:
	@echo "Installing the server"

build:
	@echo "Building the server"
	sudo docker-compose build
	@echo "Built Server"

run:
	@echo "Running the server"
	sudo docker-compose up

start:
	@echo "Starting the server"
	sudo docker-compose run -d

stop:
	@echo "Stopping the server"
	sudo docker-compose down