<div align="center">

# minecraft-registry-api

**Typed Rust clients for the Minecraft modding and server APIs**

[![Crates.io](https://img.shields.io/crates/v/minecraft_registry_api?style=flat-square)](https://crates.io/crates/minecraft_registry_api)
[![docs.rs](https://img.shields.io/docsrs/minecraft_registry_api?style=flat-square)](https://docs.rs/minecraft_registry_api)
[![CI](https://img.shields.io/github/actions/workflow/status/SirCesarium/minecraft-registry-api/ci.yml?branch=main&style=flat-square)](https://github.com/SirCesarium/minecraft-registry-api/actions)
[![License](https://img.shields.io/github/license/SirCesarium/minecraft-registry-api?style=flat-square)](LICENSE)

</div>

`minecraft-registry-api` is a Rust library with typed clients for Modrinth, Mojang, Paper, Purpur, Fabric, Forge and NeoForge. Each module returns the platform's own JSON structs, so every response maps 1:1 to the upstream API.

## Install

```bash
cargo add minecraft_registry_api
```

The clients take a `reqwest::Client` you own and return futures that run on your runtime:

```toml
[dependencies]
minecraft_registry_api = "2"
reqwest = "0.13"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quickstart

```rust
use minecraft_registry_api::error::ApiError;
use minecraft_registry_api::modrinth::ModrinthClient;
use minecraft_registry_api::modrinth::models::{Facet, ProjectType, SearchQuery};

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let client = ModrinthClient::new(reqwest::Client::new());

    let facets = vec![vec![
        Facet::ProjectType(ProjectType::Mod),
        Facet::Category("performance".into()),
    ]];

    let result = client
        .search(SearchQuery {
            query: "",
            limit: 10,
            offset: 0,
            facets: Some(facets),
        })
        .await?;

    println!("{} hits", result.total_hits);

    Ok(())
}
```

`modrinth` and `mojang` are enabled by default. The other platforms need their feature, listed in [Cargo features](#cargo-features).

## APIs

Each feature has the same name as its module, and every client is built with `new(reqwest::Client)`.

| Feature | Client | Methods |
| --- | --- | --- |
| `modrinth` | `ModrinthClient` | `search`, `get_project`, `get_versions`, `get_version`, `get_loaders`, `get_game_versions` |
| `mojang` | `MojangClient` | `get_manifest`, `download` |
| `paper` | `PaperClient` | `get_project`, `get_version`, `get_builds_by_channel`, `get_build`, `download_build` |
| `purpur` | `PurpurClient` | `get_project`, `get_version`, `get_build`, `download_build` |
| `fabric` | `FabricClient` | `get_game_versions`, `get_loaders`, `get_installer_versions`, `download_installer` |
| `forge` | `ForgeClient` | `get_promos`, `download_installer` |
| `neoforge` | `NeoForgeClient` | `get_metadata`, `download_installer` |

Every download method has a `_to` variant that passes each chunk to a closure and returns `(content_length, total_downloaded)`.

## Cargo features

| Feature | Default | Adds |
| --- | --- | --- |
| `modrinth` | yes | Modrinth client |
| `mojang` | yes | Mojang client |
| `paper` | no | Paper client |
| `purpur` | no | Purpur client |
| `fabric` | no | Fabric client |
| `forge` | no | Forge client |
| `neoforge` | no | NeoForge client |
| `full` | no | Every client |

```bash
# A single platform
cargo add minecraft_registry_api --no-default-features --features paper

# All seven
cargo add minecraft_registry_api --features full
```

## Facets

`ModrinthClient::search` accepts typed facets in `SearchQuery::facets`. Facets in the same inner `Vec` are OR'd, and separate inner `Vec`s are AND'd.

| Facet | Sent as |
| --- | --- |
| `ProjectType(ProjectType)` | `project_type:<value>` |
| `Category(String)` | `categories:<value>` |
| `Loader(String)` | `loaders:<value>` |
| `GameVersion(String)` | `game_versions:<value>` |
| `ClientSide(Side)` | `client_side:<value>` |
| `ServerSide(Side)` | `server_side:<value>` |
| `OpenSource(bool)` | `open_source:<value>` |
| `License(String)` | `license:<value>` |
| `Author(String)` | `author:<value>` |
| `Custom(String, String)` | `<key>:<value>` |

`ProjectType` covers `Mod`, `Modpack`, `Plugin`, `Datapack`, `ResourcePack` and `Shader`. `Side` covers `Required`, `Optional` and `Unsupported`.

## Models

Each `models.rs` module exposes the serde structs of its API. There is no unified domain model; each response maps to the platform's own fields.

## Errors

Every client method returns `Result<T, ApiError>`.

| Variant | When |
| --- | --- |
| `ApiError::Http(reqwest::Error)` | The request fails, returns a non-2xx status, or the JSON body does not deserialize. |
| `ApiError::Msg(String)` | The API response cannot satisfy the request, such as a Paper build without a `server:default` artifact. |
| `ApiError::Xml(String)` | NeoForge metadata cannot be parsed. |

## Development

```bash
cargo test --all-features
cargo clippy --all-features --all-targets -- -D warnings
```

The suite runs without network access. Tests inject canned responses through the `hook` test module, and fixtures live in `tests/fixtures`.

## License

Distributed under the [MIT License](LICENSE).
