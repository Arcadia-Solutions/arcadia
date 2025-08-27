# Developer Setup

## Development Containers (Optional)

<div class="warning">

If you don't want to install another toolchain on your system, You can also use [devcontainers](https://containers.dev/) instead.
If you don't know, think of isolated minimal virtual machines that come with the tools required to build Arcadia (or anything else really).

If you have Docker (recommended!) installed and use [Visual Studio Code](https://code.visualstudio.com/), all you need is to have the [Dev Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) extension installed and "reopening your folder in a container".
You can find that option in your Command Palette, or by clicking on the new status bar item in the left bottom corner.
</div>

You can also use GitHub Codespaces to build Arcadia in the cloud without having to download anything but streams of text, although it's not as free as a local dev container.

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/Arcadia-Solutions/arcadia?quickstart=1)

It isn't required to use them but can be useful in some cases (especially if you're using an immutable OS).

## Required Tools

You need these to make meaningful contributions to Arcadia, outside the cases of documentation for example.

- [Node.js & npm](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (version 1.88.0 and higher)

## Recommended Tools

- [Prettier](https://prettier.io) for proper formatting of the frontend's code.
- [sqlx-cli](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md) for managing database related stuff, including migrations.
- [Docker](https://docs.docker.com/desktop/setup/install) for setting up dependencies. Optional but HIGHLY recommended!
- [Insomnia](https://github.com/Kong/insomnia/) for testing the backend's API. You could also use any other client if you want.

## Environment Setup

### Api

At runtime, Arcadia's backend will source environment variables to influence it's behavior. The simplest way to set these during development is to write them into a file named `.env`. A documented sample file is made available, so a quick way to get started is to use it by running `cp .env.example .env`.

### Frontend

At build time, the frontend will be hardcoded with the site's API location sourced from an enviroment variable. A documented sample file is made available, so a quick way to get started is to use it by running `cp .env.example .env`.

## Building and Running

### API

```bash
# Build the backend
cargo build -p arcadia-api

# Run the backend binary
./target/debug/arcadia-api

# For optimized builds
cargo build -p arcadia-api --release
./target/release/arcadia-api
```

### Frontend

```bash
# Install dependencies
npm install

# Build and run development server
npm run dev

# For production build
npm run build
```

## Development Workflow

### Backend Development

```bash
# For development with auto-rebuild on changes
cd backend/api
cargo run

# Build and test
cargo build -p arcadia-api
cargo test

# Code quality checks
cargo clippy --fix --allow-dirty
cargo fmt --all
```

### Frontend Development

```bash
cd frontend

# Development server with hot reload
npm run dev

# Run tests
npm run test:unit

# Lint and format
npm run lint
npm run format
```
