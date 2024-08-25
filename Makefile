.PHONY: up down

up:
	@docker compose build app
	@docker compose up -d

down:
	@docker compose down