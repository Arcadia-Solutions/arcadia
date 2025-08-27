# Running Arcadia

There are two main ways to run Arcadia:

## Setup Methods

### Standard Setup
Install dependencies directly on your system. See [Standard Setup](run-standard.md) for detailed instructions.

### Docker Setup
Use containerized deployment with Docker Compose. See [Docker Setup](run-docker.md) for detailed instructions.

## Environment Configuration

Before running Arcadia, you'll need to configure environment variables. Both backend API and frontend require their own `.env` files.

Both backend API and frontend directories include `.env.example` files that you can copy:

```bash
# Backend
cd backend/api
cp .env.example .env
# Edit .env with your configuration

# Frontend
cd frontend
cp .env.example .env
# Edit .env with your configuration
```
