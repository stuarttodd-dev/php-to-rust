# 26-docker-rust-with-postgresql

![php-to-rust logo](../images/php-to-rust.png)

Rust API with PostgreSQL using Docker Compose.

This exercise demonstrates how to set up a complete development environment with a Rust web API and PostgreSQL database running together in Docker Compose.

## Prerequisites

- Docker Desktop (or Docker + Docker Compose)
- `curl` for testing endpoints
- Port 8080 available (API)
- Port 5432 available (PostgreSQL)

## Project Layout

- `Dockerfile` - Multi-stage build for the Rust binary
- `compose.yml` - Docker Compose configuration for app and database
- `src/main.rs` - Axum web API with database connection
- `.env.example` - Environment variable template
- `Cargo.toml` - Project dependencies (Axum, SQLx, Tokio, etc.)

## Step-by-Step Setup

### Step 1: Navigate to the project directory

```bash
cd 26-docker-rust-with-postgresql
```

### Step 2: Start Docker Compose

Build the Rust application and start both the app and database services:

```bash
docker compose up -d --build
```

**What this does:**
- Builds the Rust binary using the Dockerfile (multi-stage build)
- Pulls the PostgreSQL 15 Alpine image
- Creates a named volume `pgdata` for database persistence
- Starts the app container on port 8080
- Starts the database container on port 5432
- Connects them on the internal Docker network

**Expected output:**
```
✔ Network 26-docker-rust-with-postgresql_default  Created
✔ Volume "26-docker-rust-with-postgresql_pgdata"  Created
✔ Container 26-docker-rust-with-postgresql-db-1   Started
✔ Container 26-docker-rust-with-postgresql-app-1  Started
```

### Step 3: Wait for the database to be ready

The PostgreSQL database takes a moment to initialize. Wait 2-3 seconds, then check if the app is ready:

```bash
sleep 3
```

### Step 4: Test the Health Endpoint

Verify the API is running and connected to the database:

```bash
curl http://localhost:8080/health
```

**Expected response:**
```json
{"status":"OK","service":"docker-rust-with-postgresql"}
```

### Step 5: Test the Root Endpoint

Get information about available endpoints:

```bash
curl http://localhost:8080/
```

**Expected response:**
```json
{
  "message": "Rust API with PostgreSQL",
  "endpoints": [
    "/health - Health check",
    "/ - This message"
  ]
}
```

### Step 6: View Application Logs

Check that the database connected successfully:

```bash
docker compose logs app
```

**Expected output:**
```
app-1  | [2026-03-21T21:40:22Z INFO  docker_rust_with_postgresql] Database connection successful
app-1  | [2026-03-21T21:40:22Z INFO  docker_rust_with_postgresql] Server listening on 0.0.0.0:8080
```

### Step 7: View Database Logs (optional)

Verify PostgreSQL started correctly:

```bash
docker compose logs db
```

### Step 8: Stop the Stack

When you're done testing, stop the containers:

```bash
# Stop containers (data in pgdata volume persists)
docker compose down
```

Or to also delete the database data:

```bash
# Stop containers AND remove the database volume
docker compose down -v
```

## Configuration

Database connection details (defined in `compose.yml`):
- **Host**: `db` (Docker internal hostname)
- **Port**: `5432`
- **User**: `user`
- **Password**: `secret`
- **Database**: `mydb`

The app reads `DATABASE_URL` environment variable set in `compose.yml`:
```
postgres://user:secret@db:5432/mydb
```

## Troubleshooting

### "Port 8080 already in use"
Kill the existing process using port 8080 or modify the port mapping in `compose.yml`:
```yaml
ports:
  - "8081:8080"  # Access API at localhost:8081 instead
```

### "Connection refused" when testing endpoints
The app might still be starting. Wait a few seconds and check logs:
```bash
docker compose logs app
```

### "Cannot find network"
Ensure you're in the correct directory (`26-docker-rust-with-postgresql`) when running `docker compose` commands.

### "Database connection failed"
Check if PostgreSQL is running:
```bash
docker compose ps
```

All services should show `Up` status. If `db` is not up, check logs:
```bash
docker compose logs db
```

## Docker Compose Commands Cheat Sheet

```bash
# Start services in background
docker compose up -d --build

# View running services
docker compose ps

# View all logs
docker compose logs

# View logs for specific service
docker compose logs app
docker compose logs db

# Stop containers (keep data)
docker compose down

# Stop containers and remove volumes (delete data)
docker compose down -v

# Rebuild without cache
docker compose build --no-cache

# Execute command in running container
docker compose exec app command_here
```

## Key Concepts

- **Multi-stage Docker build**: Builder stage compiles Rust, runtime stage uses minimal debian image for smaller size
- **Docker Compose**: Orchestrates multiple services (app and database) with networking and volumes
- **Database Volume**: Named volume `pgdata` ensures PostgreSQL data persists across container restarts
- **Service Discovery**: App container reaches database via hostname `db` on the internal Docker network
- **Dependency Caching**: Dockerfile layers dependencies separately for faster rebuilds when only source code changes

