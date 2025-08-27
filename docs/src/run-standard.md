# Standard Setup

This page explains how to install and run Arcadia directly on your system without Docker containers.

## Prerequisites

Before starting, ensure you have the following installed:

- **PostgreSQL** - Database server
- **Rust & Cargo** - Required to build the backend
- **Node.js & npm** - Required to build the frontend
- **Git** - To clone the repository

For development tool installation instructions, see the [Developer Setup](dev-setup.md) guide.

## Quick Start

1. Clone the repository and navigate to it
2. Set up PostgreSQL database
3. Configure and run the backend
4. Configure and run the frontend

## Database Setup

### 1. Install PostgreSQL

Install PostgreSQL on your system:

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install postgresql postgresql-contrib
```

**macOS:**
```bash
brew install postgresql
brew services start postgresql
```

**Windows:**
Download and install from [PostgreSQL official website](https://www.postgresql.org/download/windows/).

### 2. Create Database and User

Connect to PostgreSQL and create the database:

```bash
# Connect as postgres user
sudo -u postgres psql

# Or on Windows/macOS:
psql -U postgres
```

In the PostgreSQL shell:
```sql
-- Create user
CREATE USER arcadia WITH PASSWORD 'your_secure_password';

-- Create database
CREATE DATABASE arcadia OWNER arcadia;

-- Grant privileges
GRANT ALL PRIVILEGES ON DATABASE arcadia TO arcadia;

-- Exit
\q
```

### 3. Run Database Migrations

Install the database migration tool and run migrations:

```bash
# Install sqlx-cli
cargo install sqlx-cli --no-default-features --features native-tls,postgres

# Navigate to storage directory
cd backend/storage

# Run migrations
sqlx migrate run --database-url postgresql://arcadia:your_secure_password@localhost:5432/arcadia
```

## Backend Setup

### 1. Environment Configuration

Navigate to the frontend directory and configure it:

```bash
cd backend/api
cp .env.example .env
```

Edit the `.env` file with your database configuration.

### 2. Build and Run

Build and start the backend server:

```bash
cargo run --release
```

If you encounter build errors, install the required system dependencies:

**Ubuntu/Debian:**
```bash
sudo apt-get install libssl-dev openssl pkg-config
```

**macOS:**
```bash
xcode-select --install
```

**Windows:**
Ensure you have Visual Studio Build Tools installed.

The backend will start and be accessible at `http://localhost:8080`.

## Frontend Setup

### 1. Environment Configuration

Navigate to the frontend directory and configure it:

```bash
cd frontend
cp .env.example .env
```

Edit the `.env` file to point to your backend.

### 2. Build and Run

Install dependencies and start the frontend:

```bash
npm install
npm run dev
```

The frontend will be accessible at `http://localhost:5173` (or the port shown in the terminal).

## Production Build

For production deployment:

### Backend API
```bash
cd backend/api
cargo build --release

# Or from the root directory
cargo build -p arcadia-api --release

# The binary will be in target/release/arcadia-api
```

### Frontend
```bash
cd frontend
npm run build
# Built files will be in the dist/ directory
```

## Troubleshooting

### Database Issues

**PostgreSQL not running:**
```bash
# Ubuntu/Debian
sudo systemctl start postgresql
sudo systemctl enable postgresql

# macOS
brew services start postgresql
```

**Connection errors:**
- Verify PostgreSQL is running on port 5432
- Check that the database and user exist
- Ensure the DATABASE_URL in `.env` is correct

### Build Issues

**Backend API build fails:**
- Install system dependencies listed above
- Update Rust: `rustup update`
- Clear build cache: `cargo clean`

**Frontend build fails:**
- Check Node.js version compatibility
- Clear npm cache: `npm cache clean --force`
- Delete `node_modules` and run `npm install` again

### Runtime Issues

**Backend API won't start:**
- Check the database connection
- Verify environment variables in `.env`
- Ensure migrations have been run

**Frontend can't connect to backend API:**
- Verify the backend is running on the correct port
- Check `VITE_API_URL` in `frontend/.env`

## Stopping Arcadia

To stop Arcadia:
1. Stop the frontend with `Ctrl+C` in its terminal
2. Stop the backend API with `Ctrl+C` in its terminal
3. Optionally stop PostgreSQL if you don't need it for other applications
