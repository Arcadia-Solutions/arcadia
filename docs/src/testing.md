# Testing Guide

Arcadia maintains automated test suites across both the Rust backend services and the Vue 3 frontend.

---

## 1. Backend Testing (Rust)

Backend testing is split between fast in-memory unit tests and database-backed Actix integration tests.

### Unit Tests
Unit tests live directly alongside source code in `#[cfg(test)] mod tests` modules within each crate (`shared`, `backend/api`, `backend/common`, `backend/storage`, `tracker/arcadia_tracker`, and `backend/periodic-tasks`).

Run all unit tests across the workspace:
```bash
cargo test --lib
```

To run unit tests for a specific crate:
```bash
cargo test -p arcadia-shared --lib
cargo test -p arcadia_tracker --lib
```

### Integration & API Tests
Integration tests live in `backend/api/tests/`. They use:
- **`actix_web::test`**: Simulates HTTP requests against real Actix service endpoints without binding to a host network port.
- **`sqlx::test`**: Executes tests against a test PostgreSQL database, using transaction rollbacks and SQL test fixtures (located in `backend/api/tests/fixtures/`).

To run integration tests, ensure a database is accessible via `DATABASE_URL` (or started with `docker compose up -d db`):

```bash
cargo test -p arcadia-api --test '*'
```

To run a single integration test file:
```bash
cargo test -p arcadia-api --test test_auth
cargo test -p arcadia-api --test test_torrent
```

---

## 2. Frontend Testing (Vue 3 & TypeScript)

The frontend uses modern testing frameworks integrated with Vite and npm scripts.

### Unit & Component Testing (Vitest)
Unit tests use [Vitest](https://vitest.dev/), a Vite-native test runner optimized for Vue 3 composables, Pinia stores, and utility functions (such as media parsers and formatters).

Run the unit test suite:
```bash
cd frontend
npm run test:unit
```

To run tests in watch mode during development:
```bash
cd frontend
npx vitest watch
```

### End-to-End Testing (Playwright)
Browser-level end-to-end testing is powered by [Playwright](https://playwright.dev/). Playwright launches headless browser engines (Chromium, Firefox, WebKit) to verify user authentication, browsing, search filters, and form submissions.

Run end-to-end tests:
```bash
cd frontend
npm run test:e2e
```

### Linters & Formatting
```bash
cd frontend
npm run lint
npm run format
```
