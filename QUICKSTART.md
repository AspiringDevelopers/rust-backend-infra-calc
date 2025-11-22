# Quick Start Guide

## Prerequisites

- Docker Desktop installed and running
- WSL2 or PowerShell (Windows)
- Git (optional)

## Step 1: Start Databases

Open PowerShell in the project directory and run:

```powershell
docker-compose up -d mongodb mysql minio
```

This starts:

- MongoDB on port 27017
- MySQL on port 3306
- MinIO on ports 9000 (API) and 9001 (Console)

## Step 2: Verify Databases are Running

```powershell
docker ps
```

You should see three containers running: `touchcalc-mongodb`, `touchcalc-mysql`, and `touchcalc-minio`.

## Step 3: Build the Application

```powershell
cargo build
```

This will take a few minutes the first time as it downloads and compiles all dependencies.

## Step 4: Run the Application

```powershell
cargo run
```

The application will start on `http://localhost:8080`

## Step 5: Access the Application

Open your web browser and navigate to:

- **Home Page**: http://localhost:8080
- **Login**: http://localhost:8080/login
- **Register**: http://localhost:8080/register
- **MinIO Console**: http://localhost:9001 (minioadmin/minioadmin)

## Step 6: Create Your First Account

1. Go to http://localhost:8080/register
2. Enter an email and password
3. Click "Register"
4. You'll be redirected to login
5. Login with your credentials
6. You'll see your dashboard!

## Troubleshooting

### Docker containers won't start

```powershell
# Stop all containers
docker-compose down

# Remove volumes and restart
docker-compose down -v
docker-compose up -d
```

### Build errors

```powershell
# Clean and rebuild
cargo clean
cargo build
```

### Connection errors

- Make sure all Docker containers are running: `docker ps`
- Check `.env` file has correct database URLs
- Verify ports 3306, 27017, 8080, 9000, 9001 are not in use

### MinIO bucket not found

MinIO buckets are created automatically by the application on first access.

## Development Mode

### Watch for changes and auto-reload

```powershell
# Install cargo-watch
cargo install cargo-watch

# Run with auto-reload
cargo watch -x run
```

### View logs

```powershell
# Application logs (if running in Docker)
docker-compose logs -f app

# Database logs
docker-compose logs -f mongodb
docker-compose logs -f mysql
docker-compose logs -f minio
```

### Stop everything

```powershell
# Stop all containers
docker-compose down

# Stop and remove volumes
docker-compose down -v
```

## API Testing

### Register a user

```powershell
Invoke-WebRequest -Uri http://localhost:8080/api/auth/register `
  -Method POST `
  -ContentType "application/json" `
  -Body '{"email":"test@example.com","password":"password123"}'
```

### Login

```powershell
Invoke-WebRequest -Uri http://localhost:8080/api/auth/login `
  -Method POST `
  -ContentType "application/json" `
  -Body '{"email":"test@example.com","password":"password123"}'
```

## Next Steps

- Customize the UI in `web/templates/` and `web/static/`
- Add more handlers in `src/handlers/`
- Configure MinIO for file storage
- Set up email service (SES or SMTP)
- Deploy to production

## Support

For issues, check:

1. Docker containers are running
2. `.env` file is configured correctly
3. No port conflicts
4. Check logs: `cargo run` output or `docker-compose logs`
