# Repository Guidelines

## Project Overview

Farmart is a grocery marketplace prototype. The repository contains a Vue 3 + TypeScript SPA and a Rust Axum API backed by SQLite. `DESIGN.md` is the visual source of truth and `PLAN.md` describes the phased product scope; treat unchecked items in `PLAN.md` as planned, not implemented.

## Repository Structure

- `vue-app/` — Vite frontend; `src/views/` contains Home, Product Detail, Design System and About pages.
- `vue-app/src/components/` — reusable UI components and Storybook stories; `src/services/api.ts` is the typed API client, `src/router/` owns routes and `src/assets/` contains global styles/assets.
- `vue-app/src/components/__tests__/` — Vitest unit tests; `vue-app/e2e/` — Playwright tests; `vue-app/.storybook/` — Storybook configuration.
- `backend/` — Rust crate (`farmart-backend`), edition 2024. `src/domain/` contains DTOs, `src/application/` contains `CatalogPort` and `AppError`, `src/adapters/inbound/` contains Axum routes/OpenAPI, and `src/adapters/outbound/sqlite.rs` implements the Diesel adapter.
- `backend/src/main.rs` is the composition root: it creates the SQLite schema, seeds repeatable demo data, configures CORS/logging and starts the server. There are no Diesel migration files.
- Root references: `DESIGN.md`, `PLAN.md`; `backend/mermaid.html` is served as the SQLite ER diagram.

## Run Locally

Start the API first:

```powershell
cd backend
cargo run
```

Defaults are `HOST=127.0.0.1`, `PORT=8080`, and `DATABASE_URL=farmart.db`; override them with environment variables or `backend/.env`. The API exposes `/docs`, `/openapi.json`, and `/mermaid.html`.

In another terminal:

```powershell
cd vue-app
npm.cmd install
npm.cmd run dev
```

The frontend defaults to `VITE_API_BASE_URL=http://127.0.0.1:8080/api/v1`. Playwright uses Vite on port 5173 locally and the preview server on port 4173 in CI.

## API Surface

Implemented routes are `GET /health`, `/home`, `/categories`, `/brands`, `/products`, `/products/{id}`, `/products/{id}/detail`, plus `POST /auth/register` and `POST /cart/items`. Product filters are `q`, `category_id`, `sort`, `page` and `per_page`. Keep DTO changes, route annotations and the generated Utoipa schema synchronized. Registration and cart are currently validation/demo handlers; they do not persist members or cart contents.

## Verification

Run the relevant lint checks whenever adding or changing a feature. Keep frontend and
backend checks separate:

From `backend/`: `check-format.cmd` (the environment blocks `cargo-fmt.exe`, so this invokes the
allowed toolchain `rustfmt.exe` directly), `cargo clippy --all-targets --all-features -- -D warnings`,
`cargo check`, `cargo test`, and `cargo build`.

From `vue-app/`: `npm.cmd run lint` (ESLint, Oxlint and Prettier), then the relevant tests/builds.

From `vue-app/`: `npm.cmd run type-check`, `npm.cmd run build`, `npm.cmd run test:unit`, `npm.cmd run test:e2e`, `npm.cmd run format`, and `npm.cmd run build-storybook`. E2E projects are Chromium, Firefox and WebKit. Run a focused Playwright test with `npm.cmd run test:e2e -- --project=chromium`.

## Coding Conventions

Use two-space indentation, single quotes and no semicolons in Vue/TypeScript; these are enforced by the frontend Prettier and ESLint/Oxlint checks. Rust formatting is enforced by `backend/rustfmt.toml` and `backend/check-format.cmd`. Use PascalCase component names, `*View.vue` for route pages and explicit TypeScript types. Prefer scoped styles, tokens from `DESIGN.md` and `AppIcon`. Keep domain/application code independent of Axum and Diesel; connect infrastructure through ports/adapters. Add or update unit, integration or E2E coverage for behavior changes.

## Change and Security Guidelines

Do not commit `.env`, credentials, `backend/*.db`, Rust build/coverage output (`target/`, `target-*/`, `coverage.info`) or generated frontend output (`dist/`, `storybook-static/`). Keep `Cargo.lock`, `package.json` and `package-lock.json` synchronized. API changes must update Utoipa annotations/schema and backend documentation. UI changes should include screenshots or a short recording in the PR description. The repository root has no Git history/metadata, so no local commit-message convention can be inferred; use concise imperative messages if commits are introduced.

## graphify

This project has a knowledge graph at graphify-out/ with god nodes, community structure, and cross-file relationships.

When the user types `/graphify`, use the installed graphify skill or instructions before doing anything else.

Rules:
- For codebase questions, first run `graphify query "<question>"` when graphify-out/graph.json exists. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts. These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- Dirty graphify-out/ files are expected after hooks or incremental updates; dirty graph files are not a reason to skip graphify. Only skip graphify if the task is about stale or incorrect graph output, or the user explicitly says not to use it.
- If graphify-out/wiki/index.md exists, use it for broad navigation instead of raw source browsing.
- Read graphify-out/GRAPH_REPORT.md only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).
