db-up:
	docker compose up db -d
db-down:
	docker compose down db

local-run:
	cargo run
