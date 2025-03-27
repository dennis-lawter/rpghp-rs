banner := $(shell tput bold)$(shell tput setab 4)
reset := $(shell tput sgr0)

# DB
db-up:
	docker compose up db -d

db-down:
	docker compose down db

# DEV
local-run: cargo-clippy cargo-build cargo-run

cargo-clippy:
	@printf "$(banner)Code scan running...$(reset)\\n"
	cargo clippy -- -D warnings

cargo-build:
	@printf "$(banner)Project building...$(reset)\\n"
	cargo build

cargo-run:
	@printf "$(banner)Running...$(reset)\\n"
	cargo run

hot-reload:
	@printf "$(banner)Hot reload activating...$(reset)\\n"
	cargo watch --poll -x "clippy -- -D warnings" -x "build" -x "run"
