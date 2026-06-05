# minecraft-registry-api

[![Crates.io](https://img.shields.io/crates/v/minecraft_registry_api?style=flat-square)](https://crates.io/crates/minecraft_registry_api)
[![CI](https://img.shields.io/github/actions/workflow/status/SirCesarium/minecraft-registry-api/ci.yml?branch=main&style=flat-square)](https://github.com/SirCesarium/minecraft-registry-api/actions)
[![License](https://img.shields.io/github/license/SirCesarium/minecraft-registry-api?style=flat-square)]()

Typed API clients for Minecraft modding platforms — Modrinth, Mojang, Paper, Purpur, Fabric, Forge, NeoForge.

```rust
use minecraft_registry_api::modrinth::ModrinthClient;
use minecraft_registry_api::modrinth::models::{Facet, SearchQuery};

let client = ModrinthClient::new(reqwest::Client::new());
let result = client.search(SearchQuery { query: "sodium", limit: 10, offset: 0, facets: None }).await?;
println!("{} hits", result.total_hits);

// Faceted search
use Facet::{ProjectType, Category};
let facets = vec![vec![ProjectType(ProjectType::Mod), Category("performance".into())]];
let result = client.search(SearchQuery { query: "", limit: 10, offset: 0, facets: Some(facets) }).await?;
```

```rust
use minecraft_registry_api::mojang::MojangClient;

let client = MojangClient::new(reqwest::Client::new());
let manifest = client.get_manifest().await?;
let latest = manifest.versions.iter()
    .find(|v| v.id == manifest.latest.release)
    .unwrap();
```

```rust
use minecraft_registry_api::forge::{ForgeClient, models::InstallerQuery};

let client = ForgeClient::new(reqwest::Client::new());
let promos = client.get_promos().await?;
let jar = client.download_installer(InstallerQuery { mc_version: "1.21.4", forge_version: "54.1.16" }).await?;
```

## APIs

Each API is behind an optional Cargo feature:

| Feature | Module | Client | Methods |
|---------|--------|--------|---------|
| `modrinth` | `modrinth` | `ModrinthClient` | `search`, `get_project`, `get_versions`, `get_version`, `get_loaders`, `get_game_versions` |
| `mojang` | `mojang` | `MojangClient` | `get_manifest`, `download` |
| `paper` | `paper` | `PaperClient` | `get_project`, `get_build`, `download_build` |
| `purpur` | `purpur` | `PurpurClient` | `get_project`, `get_version`, `download_build` |
| `fabric` | `fabric` | `FabricClient` | `get_game_versions`, `get_loaders`, `get_installer_versions`, `download_installer` |
| `forge` | `forge` | `ForgeClient` | `get_promos`, `download_installer` |
| `neoforge` | `neoforge` | `NeoForgeClient` | `get_metadata`, `download_installer` |

```toml
# Default: modrinth + mojang
cargo add minecraft_registry_api

# Single API
cargo add minecraft_registry_api --no-default-features --features paper

# All APIs
cargo add minecraft_registry_api --features full
```

### Modrinth facets

`ModrinthClient::search` accepts facets — typed filters that narrow results:

```rust
use minecraft_registry_api::modrinth::models::{Facet, ProjectType, Side};

let facets = vec![vec![
    Facet::ProjectType(ProjectType::Modpack),
    Facet::Category("technology".into()),
    Facet::Loader("fabric".into()),
    Facet::GameVersion("1.21.4".into()),
    Facet::ClientSide(Side::Required),
    Facet::ServerSide(Side::Required),
    Facet::OpenSource(true),
    Facet::License("MIT".into()),
    Facet::Author("CaffeineMC".into()),
    Facet::Custom("key".into(), "value".into()),
]];
```

Multiple facets in the same inner `Vec` are OR'd. Multiple inner `Vec`s are AND'd.

### Models

Models map 1:1 to each API's JSON. There is no unified domain model — that belongs in the consumer (e.g. conduit-core). Each `models.rs` module exposes the raw serde structs.

### Error handling

All methods return `Result<T, minecraft_registry_api::error::ApiError>`:

- `ApiError::Http(reqwest::Error)` — request failed or returned a non-2xx status
- `ApiError::Xml(quick_xml::de::DeError)` — XML deserialization (forge, neoforge)

`ApiError` implements `From<reqwest::Error>` and `std::error::Error`.

## Install

```bash
cargo add minecraft_registry_api
```

Then enable the features you need (modrinth and mojang are on by default).

## Tests

Tests hit real APIs. No mocks, no fixtures.

```bash
cargo test --features full
```

Clippy lints deny `unwrap_used` and `expect_used` across the library. Test modules use `#[allow(clippy::unwrap_used)]`.

## License

MIT
