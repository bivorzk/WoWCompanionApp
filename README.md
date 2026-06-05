# WoWCompanionApp

WoWCompanionApp is a desktop-first World of Warcraft companion built in Rust. I am getting the data from Raider.IO, Blizzard profile pages,Blizzard API, warcraftlogs and other things. This project is made for fun/personal use it's current for retail wow only  

## Current Status


What works today:

- Character search using `CharacterName-Realm`
- Region switching for desktop lookups
- Recent search history and favorites-backed search dropdown
- Home, Overall, Character, Favorites, and Settings routes
- Live character profile loading from Raider.IO and Blizzard
- PvP, Raids, Mythic+, Endgame, and Collection profile tabs
- Gear inspector, achievements feed, faction badges, and profile theming

## Live Data Sources

The app currently uses these sources:

- Blizzard Battle.net API for equipment, achievements, PvP profile data, and collection data when the public profile exposes it
- Raider.IO public profile data for Mythic+ score and recent public runs
- Raider.IO site overview data for raid progression and richer character detail
- Warcraft Logs API client code exists in `packages/api`, but its data is not yet surfaced in the desktop UI

Note on collections:

- Blizzard collection endpoints are not uniformly exposed for every public profile.
- The desktop UI now treats restricted or unavailable mount and pet data explicitly instead of rendering a misleading `0`.

## Tech Stack

- Rust workspace
- Dioxus `0.7.4` for the desktop UI
- Tokio and reqwest for async networking
- Shared API clients in `packages/api`
- Repository + Service + Signal Store organization in the desktop crate

## Repository Layout

Active workspace members:

```text
packages/
├─ api/       # Blizzard, Raider.IO, Warcraft Logs, and scraping helpers
├─ db/        # It would have been used for database stuff but I changed my mind so it's currently a placeholder
├─ desktop/   # Dioxus desktop application
└─ web/       # Placeholder crate for a future web target
```

## Current Desktop Routes

The desktop app currently exposes these routes:

- `/`
- `/overall`
- `/character/:region/:realm/:name`
- `/favorites`
- `/settings`

## Getting Started

### Prerequisites

- Rust toolchain
- Optional: Dioxus CLI for desktop development

### 1. Configure environment variables

Create a local `.env` from the example file:

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

Notes:

- Raider.IO does not require credentials.
- Blizzard and Warcraft Logs credentials are loaded via `.env` fallback in the API crate.

### 2. Start PostgreSQL

The repository includes a local development database definition:

```powershell
docker compose up -d
```

### 3. Build the workspace

```powershell
cargo build
```

### 4. Validate the current desktop slice

```powershell
cargo check -p desktop
```

### 5. Run the desktop app

Direct Cargo run:

```powershell
cargo run -p desktop
```

Dioxus CLI workflow:

```powershell
dx run --package desktop
```
```
or just go to the release tab and download it there yourself without having to compiling it

```

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE).


