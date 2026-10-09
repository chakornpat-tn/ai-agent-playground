# Farmart: Learning AI Agent Coding Through a Full-Stack Project

Farmart is a grocery marketplace prototype and a hands-on learning project for building software with AI coding agents. The goal is to learn the complete agent-assisted development loop: understand a codebase, turn product ideas into scoped work, implement changes across a frontend and backend, verify them, and keep documentation and architecture aligned.

This repository is the learning environment. The grocery marketplace gives the exercises a concrete domain; the central subject is how to work effectively with AI coding agents and how to review the code they produce.

## What you will learn

- How to give an agent repository context, constraints, and a clear definition of done.
- How to explore an existing application before proposing or making changes.
- How to break product work into small, reviewable implementation steps.
- How to coordinate a Vue frontend with a Rust HTTP API and SQLite data layer.
- How to use project plans, design references, API contracts, and architecture boundaries as working context.
- How to inspect generated changes, catch incorrect assumptions, and request targeted revisions.
- How to verify changes with formatting, linting, type checks, tests, and builds.
- How to keep implementation, API documentation, and the repository knowledge graph in sync.

An AI agent is a coding collaborator, not an authority. Treat its output as a proposed change: check that it matches the request, follows the project conventions, handles relevant edge cases, and passes the appropriate checks.

## The project at a glance

Farmart is a grocery marketplace prototype with a Vue 3 and TypeScript single-page application and a Rust Axum API backed by SQLite.

| Area | Location | Role |
| --- | --- | --- |
| Frontend | `vue-app/` | Vue 3, TypeScript, Vite application and UI components |
| Backend | `backend/` | Rust 2024 crate with Axum routes and SQLite persistence |
| Product scope | `PLAN.md` | Phased feature plan; unchecked items are planned, not implemented |
| Visual direction | `DESIGN.md` | Source of truth for the application's visual language |
| Knowledge graph | `graphify-out/` | Code and documentation relationships for graph-based exploration |

The backend is organized around domain and application code, inbound HTTP adapters, and an outbound SQLite adapter. `backend/src/main.rs` composes these pieces, creates the schema, seeds repeatable demo data, and starts the server. There are no Diesel migration files.

The frontend includes Home, Product Detail, Design System, and About views. Its typed API client lives at `vue-app/src/services/api.ts`; routes are defined in `vue-app/src/router/`.

## Getting started

### Prerequisites

- Rust toolchain with Cargo
- Node.js and npm
- Windows PowerShell for the repository's documented `.cmd` scripts

### Start the API

From `backend/`:

```powershell
cargo run
```

Defaults are `HOST=127.0.0.1`, `PORT=8080`, and `DATABASE_URL=farmart.db`. You can override them with environment variables or `backend/.env`.

The API serves `/health`, `/docs`, `/openapi.json`, and `/mermaid.html`. Its versioned API base path is `/api/v1`.

### Start the frontend

In another terminal, from `vue-app/`:

```powershell
npm.cmd install
npm.cmd run dev
```

The frontend defaults to `VITE_API_BASE_URL=http://127.0.0.1:8080/api/v1`.

## A practical AI agent coding workflow

Use the project to practice a repeatable loop rather than asking an agent for a large, vague change all at once.

1. **Orient.** Read the relevant files, `PLAN.md`, `DESIGN.md`, and local `AGENTS.md` instructions. For codebase questions, use the Graphify knowledge graph when it is available.
2. **Define the task.** State the user-visible outcome, relevant constraints, and what should count as complete. Identify the frontend, backend, or documentation surfaces that may be affected.
3. **Explore before editing.** Trace the existing flow and contracts. For an API change, inspect the DTO, route, application port, adapter, and OpenAPI representation as applicable. For a UI change, inspect the view, shared components, styles, and API client.
4. **Make a focused change.** Ask the agent to work within the existing architecture and conventions. Keep changes understandable and reviewable.
5. **Review the diff.** Confirm the implementation answers the original request, preserves existing behavior, handles invalid or empty input where relevant, and does not introduce unrelated changes.
6. **Verify.** Run the relevant formatter, linter, type checker, tests, or build. Review failures rather than treating a successful edit as proof of correctness.
7. **Update project knowledge.** Keep docs and API schemas aligned with code. After code changes, run `graphify update .` to refresh the repository graph.

Useful prompts are specific about both the task and the boundaries. For example:

> Trace how product sorting currently works from the Vue filter to the API query. Explain the relevant files and suggest the smallest change needed to add a sort option. Do not edit files yet.

Then, after agreeing on the approach:

> Add the selected sort option using the existing API and UI patterns. Update relevant documentation or tests, and report the files changed and checks run.

## Implemented API surface

The current routes include:

- `GET /health`
- `GET /api/v1/home`
- `GET /api/v1/categories`
- `GET /api/v1/brands`
- `GET /api/v1/products`
- `GET /api/v1/products/{id}`
- `GET /api/v1/products/{id}/detail`
- `POST /api/v1/auth/register`
- `POST /api/v1/cart/items`

Product listing accepts `q`, `category_id`, `sort`, `page`, and `per_page` filters. Registration and cart routes are currently validation/demo handlers; they do not persist members or cart contents.

When changing API behavior, keep DTOs, route annotations, and the generated Utoipa schema synchronized. Update backend documentation as part of the same change.

## Verification commands

Run checks for the part of the project you changed. The repository's full check catalog is below.

### Backend

From `backend/`:

```powershell
check-format.cmd
cargo clippy --all-targets --all-features -- -D warnings
cargo check
cargo test
cargo build
```

`check-format.cmd` invokes the allowed Rust formatter directly because this environment blocks `cargo-fmt.exe`.

### Frontend

From `vue-app/`:

```powershell
npm.cmd run lint
npm.cmd run type-check
npm.cmd run test:unit
npm.cmd run build
```

Other available scripts include `npm.cmd run test:e2e`, `npm.cmd run format`, and `npm.cmd run build-storybook`. Playwright E2E projects cover Chromium, Firefox, and WebKit. To run only Chromium:

```powershell
npm.cmd run test:e2e -- --project=chromium
```

## Conventions and boundaries

- Vue and TypeScript use two-space indentation, single quotes, and no semicolons. ESLint, Oxlint, and Prettier enforce the frontend conventions.
- Rust formatting follows `backend/rustfmt.toml` and `backend/check-format.cmd`.
- Use PascalCase Vue component names and `*View.vue` for route pages. Prefer explicit TypeScript types and scoped styles.
- Use design tokens from `DESIGN.md` and the shared `AppIcon` component.
- Keep domain and application code independent of Axum and Diesel. Connect infrastructure through ports and adapters.
- Do not assume planned items in `PLAN.md` already exist in the application.

## Repository hygiene

Do not commit `.env` files, credentials, `backend/*.db`, Rust build or coverage output (`target/`, `target-*/`, `coverage.info`), or generated frontend output (`dist/`, `storybook-static/`). Keep `Cargo.lock`, `package.json`, and `package-lock.json` synchronized when changing dependencies.

## Learning path

1. **Explore a feature:** trace a product from the Home view to its API endpoint and backend adapter.
2. **Make a small UI change:** adjust a component using the design references and verify it with frontend checks.
3. **Make a contract change:** update a DTO, route, schema, and matching client types together.
4. **Practice agent review:** ask an agent to explain its own diff and identify assumptions or uncovered cases, then verify those claims yourself.
5. **Take a planned feature:** select an unchecked item from `PLAN.md`, define acceptance criteria, and implement it in small steps.

The best learning outcome is not simply a working feature. It is being able to explain how the change fits the system, why the agent's approach is sound, and what evidence shows that the implementation works.
