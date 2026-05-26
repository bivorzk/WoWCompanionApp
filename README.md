# WoWCompanionApp

WoWCompanionApp is a desktop-first World of Warcraft companion built in Rust. The goal is to surface character data, Mythic+ performance, raid progression, and log context in a native app instead of making players bounce between multiple browser tabs.

## Status

The repository is in early development. The desktop shell, routing, API crates, and database crate are in place, while the richer character views and external data integrations are still being implemented.

## Planned Features

- Character search by region, realm, and name
- Blizzard character profile, gear, and progression data
- Raider.IO Mythic+ scores and recent run history
- Warcraft Logs integration for parse and encounter context
- Favorites and historical snapshots backed by PostgreSQL
- Shared Rust code that can support a future web target

## Tech Stack

- Rust workspace
- Dioxus 0.7.4 for the desktop UI
- Tokio and reqwest for async networking
- sqlx with PostgreSQL for persistence
- Reusable API client code in `packages/api`

## Repository Layout

Active workspace members:

```text
packages/
├─ api/       # Blizzard, Raider.IO, Warcraft Logs, and scraping helpers
├─ db/        # PostgreSQL access and migrations
├─ desktop/   # Dioxus desktop application
└─ web/       # Placeholder crate for a future web target
```

Additional scaffolds kept from the template:

```text
packages/
├─ mobile/    # Future mobile target scaffold
└─ ui/        # Future shared UI crate scaffold
```

## External Services

The project is designed around these data sources:

- Blizzard Battle.net API
- Raider.IO API
- Warcraft Logs API v2
- Optional scraping targets for supplemental ranking or meta data

Raider.IO does not require credentials. Blizzard and Warcraft Logs do.

## Getting Started

### Prerequisites

- Rust toolchain
- Docker Desktop or a local PostgreSQL instance
- Optional: Dioxus CLI for a faster desktop dev loop

### 1. Configure environment variables

Create a local `.env` from the example file and fill in your credentials:

```powershell
Copy-Item .env.example .env
```

Required variables:

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/wowcompanion
BLIZZARD_CLIENT_ID=
BLIZZARD_CLIENT_SECRET=
WCL_CLIENT_ID=
WCL_CLIENT_SECRET=
```

### 2. Start PostgreSQL

The repository includes a local development database definition:

```powershell
docker compose up -d
```

### 3. Build the workspace

```powershell
cargo build
```

### 4. Run the desktop app

For a direct Cargo run:

```powershell
cargo run -p desktop
```

For a Dioxus-driven dev workflow:

```powershell
dx run --package desktop
```

## Current Desktop Routes

The desktop crate already includes the initial route structure for:

- `/`
- `/character/:region/:realm/:name`
- `/favorites`
- `/settings`

## Development Notes

- The root workspace currently includes `api`, `db`, `desktop`, and `web`.
- `mobile` and `ui` are present in the repository as future scaffolds, but are not wired into the root workspace yet.
- `.env.example` is safe to commit. Your local `.env` should stay untracked.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE).


