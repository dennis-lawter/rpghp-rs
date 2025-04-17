include Makehelp.mk

banner := $(shell tput bold)$(shell tput setab 4)
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
