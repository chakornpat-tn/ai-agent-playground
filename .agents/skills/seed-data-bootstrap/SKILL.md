---
name: seed-data-bootstrap
description: Seed deterministic starting data into a project so development can begin immediately. Use this skill whenever a user asks to add seed data, sample data, demo data, fixtures, bootstrap data, initialize a database, or make a new project usable locally. Inspect the existing schema, migrations, repositories, and current UI/API examples first; preserve the project's domain language and make the seed repeatable without internet access.
compatibility: Requires the project's existing database tooling or a local runtime such as Rust, Node.js, Python, or SQL CLI. Network access is not required.
---

# Seed Data Bootstrap

Use this skill to turn the current project into a ready-to-develop local environment with realistic, deterministic data.

## Workflow

1. Inspect the project before editing.
   - Read repository instructions such as `AGENTS.md`, `README.md`, schema/migration files, and the current API or UI data consumers.
   - Identify the database engine, startup command, schema creation path, and the existing seed path.
   - Treat existing domain models, routes, and displayed data as the source of truth. Do not invent a parallel schema.

2. Define the minimum useful dataset.
   - Seed parent entities before children and satisfy every foreign key and required field.
   - Include enough records for the current development flows: list pages, search/filter/sort, detail pages, empty/error states where applicable, and representative optional values.
   - Use stable slugs, IDs only when the schema requires them, deterministic timestamps, and realistic values from the product domain.
   - Never seed passwords, tokens, credentials, personal data, or production records.

3. Implement an idempotent seed path.
   - Prefer the project's existing startup initializer, migration seed, fixture loader, or repository abstraction.
   - Use `INSERT ... ON CONFLICT`, `INSERT OR IGNORE`, upserts, or a clear reset-and-reseed command according to the database's conventions.
   - Keep domain/application code independent from the database adapter where the architecture already has ports and adapters.
   - Do not download images, packages, or remote data. Reuse existing local assets or stable URLs already present in the project.

4. Verify locally.
   - Run the project's formatter, type/compiler checks, and tests.
   - Start the local API or execute the seed command against a disposable/local database.
   - Query each seeded entity and exercise the key API/UI flow. Run the seed a second time and confirm there are no duplicate rows or errors.
   - Check that tracked database files, secrets, build output, and generated artifacts remain ignored.

5. Report the result.
   - List the seed command or startup behavior, entities/counts, idempotency behavior, and verification commands.
   - Mention any intentionally omitted production-like data or any local dependency the developer must provide.

## Farmart project reference

For this repository, the current bootstrap path is `backend/src/main.rs::init_db`, called by `cargo run` before the Axum server starts. Keep the seed compatible with the SQLite schema and the API contract in `backend/src/adapters/inbound/http.rs`.

Read [references/farmart-seed-reference.md](references/farmart-seed-reference.md) before changing Farmart seed data. It records the currently displayed domain entities and the flows the initial dataset must support.

Use a disposable SQLite database for verification when possible. Do not modify or commit `backend/farmart.db` as part of ordinary development work.

## Offline and safety rules

- The seed workflow must work with internet access disabled after dependencies are already installed.
- Do not call external APIs to obtain seed data.
- Do not delete a user's database unless they explicitly request a reset and the exact target is confirmed.
- Keep seed values obviously non-production and document any destructive reset separately from the normal idempotent command.

## Completion checklist

- [ ] Existing schema and seed path inspected.
- [ ] Required parent/child relationships are valid.
- [ ] Seed is deterministic and repeatable.
- [ ] Main development flows have representative records.
- [ ] No credentials or production data are included.
- [ ] Formatter, build/check, tests, and a second seed run pass.
- [ ] README/developer instructions mention how to start with seeded data.
