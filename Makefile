.PHONY: help build up down restart logs clean test

help:
	@echo "Available commands:"
	@echo "  make build      - Build all Docker images"
	@echo "  make up         - Start all services"
	@echo "  make down       - Stop all services"
	@echo "  make restart    - Restart all services"
	@echo "  make logs       - View logs from all services"
	@echo "  make clean      - Stop services and remove volumes"
	@echo "  make test       - Run tests in container"
	@echo "  make shell      - Open shell in backend container"
	@echo "  make db-shell   - Open MongoDB shell"
	@echo "  make mysql-shell - Open MySQL shell"
	@echo "  make status     - Show service status"

build:
	docker-compose build

up:
	docker-compose up -d

down:
	docker-compose down

restart:
	docker-compose restart

logs:
	docker-compose logs -f

logs-backend:
	docker-compose logs -f rust-backend

clean:
	docker-compose down -v
	docker system prune -f

test:
	docker-compose exec rust-backend cargo test

shell:
	docker-compose exec rust-backend sh

db-shell:
	docker-compose exec mongodb mongosh touchcalc

mysql-shell:
	docker-compose exec mysql mysql -u root -ppassword touchcalc

status:
	docker-compose ps

rebuild:
	docker-compose up -d --build

health:
	@echo "Checking service health..."
	@curl -s http://localhost:8080/health || echo "Backend not responding"
	@curl -s http://localhost:9000/minio/health/live || echo "MinIO not responding"
