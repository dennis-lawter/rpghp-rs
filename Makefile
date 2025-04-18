include Makehelp.mk

banner := $(shell tput bold)$(shell tput setab 4)
releasebanner := $(shell tput bold)$(shell tput setab 5)
reset := $(shell tput sgr0)

########################################
#   Database                           #
########################################

## @Database Run the postgres container
db-up:
	docker compose up db -d

## @Database Drop the postgres container
db-down:
	docker compose down db

########################################
#   Devtools                          #
########################################

## @Devtools Run the server locally
local-run: cargo-clippy cargo-build cargo-run

## @Devtools Run the server locally with hot reloading
hot-reload:
	@printf "$(banner)Hot reload activating...$(reset)\\n"
	cargo watch --poll -x "clippy -- -D warnings" -x "build" -x "run"

########################################
#   Buildchain                         #
########################################

## @Buildchain Use clippy to analyze the code for issues
cargo-clippy:
	@printf "$(banner)Code scan running...$(reset)\\n"
	cargo clippy -- -D warnings

## @Buildchain Build the server executable
cargo-build:
	@printf "$(banner)Project building...$(reset)\\n"
	cargo build

## @Buildchain Run the server executable
cargo-run:
	@printf "$(banner)Running...$(reset)\\n"
	cargo run

## @Buildchain Prepare a report of macro checking
cargo-sqlx-prep:
	@printf "$(banner)SQL Preparation in progress...$(reset)\\n"
	cargo sqlx prepare

## @Buildchain Build the docker image for the app
docker-build:
	@printf "$(banner)Building docker image...$(reset)\\n"
	docker compose build app

########################################
#   Docker                             #
########################################

## @Docker Runs the latest built docker image for the app
docker-up:
	@printf "$(banner)Starting docker container...$(reset)\\n"
	docker compose up -d app

## @Docker Brings any running instance of docker down
docker-down:
	@printf "$(banner)Stopping docker container...$(reset)\\n"
	docker compose up -d app

########################################
#   RELEASE                            #
########################################

## @Release 
release: release-pre-banner db-up docker-build docker-down docker-up release-post-banner
release-pre-banner:
	@printf "$(releasebanner)REBUILDING AND RESTARTING SERVER$(reset)\\n"
release-post-banner:
	@printf "$(releasebanner)SERVER IS LIVE$(reset)\\n"
